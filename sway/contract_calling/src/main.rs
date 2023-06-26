use fuels::{prelude::*, accounts::wallet::WalletUnlocked};
use rust_mnist::Mnist;

abigen!(Contract(
    name = "OCRContract",
    abi = "../out/debug/sway-abi.json"
));

fn divide_ufp32(a: UFP32) -> UFP32 {
    // Divides by 256
    let denominator = (1u32 << 16) as u64;

    let self_u64: u64 = a.value as u64;
    let divisor_u64: u64 = 256;

    let res_u64 = self_u64 * denominator / divisor_u64;
    UFP32 {
        value: res_u64 as u32,
    }
}

#[tokio::main]
async fn main() {
    let wallet = launch_provider_and_get_wallet().await;
    let contract_id = Contract::load_from("../out/debug/sway.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = OCRContract::new(contract_id, wallet);
    instance.methods().init_network(vec![784, 10], IFP64 { underlying: UFP32 { value: 1u32 << 16 }, non_negative: true }).tx_params(TxParameters::default().set_gas_limit(10_000_000)).call().await.unwrap();

    let mnist = Mnist::new(r"data/");
    let mut inputs: Vec<Vec<IFP64>> = mnist
        .train_data
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| IFP64 {
                    underlying: divide_ufp32(UFP32 {
                        value: 1u32 << 16 * *y as u32,
                    }),
                    non_negative: true,
                })
                .collect()
        })
        .collect();
    let mut targets: Vec<Vec<IFP64>> = mnist
        .train_labels
        .iter()
        .map(|x| {
            (0..10)
                .map(|y| IFP64 {
                    underlying: UFP32 {
                        value: 1u32 << 16 * (y == *x as u32) as u32,
                    },
                    non_negative: true,
                })
                .collect()
        })
        .collect();

    inputs.truncate(5000);
    targets.truncate(5000);

    let test_inputs: Vec<Vec<IFP64>> = mnist
        .test_data
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| IFP64 {
                    underlying: divide_ufp32(UFP32 {
                        value: 1u32 << 16 * *y as u32,
                    }),
                    non_negative: true,
                })
                .collect()
        })
        .collect();
    let test_targets: Vec<Vec<IFP64>> = mnist
        .test_labels
        .iter()
        .map(|x| {
            (0..10)
                .map(|y| IFP64 {
                    underlying: UFP32 {
                        value: 1u32 << 16 * (y == *x as u32) as u32,
                    },
                    non_negative: true,
                })
                .collect()
        })
        .collect();

    let correct = accuracy(&instance, test_inputs.clone(), test_targets).await;
    println!("Accuracy: {}/{}", correct, test_inputs.len());
}

async fn accuracy(instance: &OCRContract<WalletUnlocked>, test_data: Vec<Vec<IFP64>>, test_labels: Vec<Vec<IFP64>>) -> i32 {
    let mut correct = 0;
    for i in 0..test_data.len() {
        let output = instance.methods().feed_forward(test_data[i].to_owned()).call().await.unwrap().value;
        let mut max = 0;
        let mut max_index = 0;
        for j in 0..output.len() {
            if output[j].underlying.value > max {
                max = output[j].underlying.value;
                max_index = j;
            }
        }
        if test_labels[i][max_index].underlying.value == 1u32 << 16 {
            correct += 1;
        }
    }
    correct
}