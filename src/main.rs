use activations::SIGMOID;
use network::Network;
use rust_mnist::Mnist;

pub mod activations;
pub mod matrix;
pub mod network;

use std::time;

fn main() {
    let mnist = Mnist::new(r"data\");
    let mut inputs: Vec<Vec<f64>> = mnist.train_data.iter().map(|x| x.iter().map(|y| *y as f64 / 256.0).collect()).collect();
    let mut targets: Vec<Vec<f64>> = mnist.train_labels.iter().map(|x| {
        let mut vec = vec![0.0; 10];
        vec[*x as usize] = 1.0;
        vec
    }).collect();
    
    inputs.truncate(35000);
    targets.truncate(35000);

    let test_inputs: Vec<Vec<f64>> = mnist.test_data.iter().map(|x| x.iter().map(|y| *y as f64 / 256.0).collect()).collect();
    let test_targets: Vec<Vec<f64>> = mnist.test_labels.iter().map(|x| {
        let mut vec = vec![0.0; 10];
        vec[*x as usize] = 1.0;
        vec
    }).collect();

    let mut network = Network::new(vec![784, 10, 15, 10], 0.015, SIGMOID);

    let correct = accuracy(&mut network, test_inputs, test_targets.clone());

    println!("Accuracy: {}/{}", correct, test_targets.len());

    // time before training
    let start = time::Instant::now();
    network.train(inputs.clone(), targets, 1);
    // time after training
    let end = time::Instant::now();
    println!("Time taken: {}s", (end - start).as_secs());
    // network.batch_train(training_data, 10, 10);

    //count how many test samples are correctly classified
    let correct = accuracy(&mut network, test_inputs, test_targets.clone());

    println!("Accuracy: {}/{}", correct, test_targets.len());

    //save the nn
    network.save("nn.json".to_string());
}

fn accuracy(nn: &mut Network, test_data: Vec<Vec<f64>>, test_labels: Vec<Vec<f64>>) -> f64 {
    let mut correct = 0;
    for i in 0..test_data.len() {
        let output = nn.feed_forward(test_data[i].to_owned());
        let mut max = 0.0;
        let mut max_index = 0;
        for j in 0..output.len() {
            if output[j] > max {
                max = output[j];
                max_index = j;
            }
        }
        if test_labels[i][max_index] == 1.0 {
            correct += 1;
        }
    }
    correct as f64 / test_data.len() as f64
}