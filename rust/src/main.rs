use activations::SIGMOID;
use network::Network;
// use rust_mnist::Mnist;

pub mod activations;
pub mod matrix;
pub mod network;

// use std::time;

// fn main() {
//     let mnist = Mnist::new(r"data\");
//     let mut inputs: Vec<Vec<f64>> = mnist.train_data.iter().map(|x| x.iter().map(|y| *y as f64 / 256.0).collect()).collect();
//     let mut targets: Vec<Vec<f64>> = mnist.train_labels.iter().map(|x| {
//         let mut vec = vec![0.0; 10];
//         vec[*x as usize] = 1.0;
//         vec
//     }).collect();
    
//     inputs.truncate(35000);
//     targets.truncate(35000);

//     let test_inputs: Vec<Vec<f64>> = mnist.test_data.iter().map(|x| x.iter().map(|y| *y as f64 / 256.0).collect()).collect();
//     let test_targets: Vec<Vec<f64>> = mnist.test_labels.iter().map(|x| {
//         let mut vec = vec![0.0; 10];
//         vec[*x as usize] = 1.0;
//         vec
//     }).collect();

//     let mut network = Network::new(vec![784, 10, 15, 10], 0.015, SIGMOID);

//     let correct = accuracy(&mut network, test_inputs.clone(), test_targets.clone());

//     println!("Accuracy: {}/{}", correct, test_targets.len());

//     // time before training
//     let start = time::Instant::now();
//     network.train(inputs.clone(), targets, 1);
//     // time after training
//     let end = time::Instant::now();
//     println!("Time taken: {}s", (end - start).as_secs());
//     // network.batch_train(training_data, 10, 10);

//     //count how many test samples are correctly classified
//     let correct = accuracy(&mut network, test_inputs.clone(), test_targets.clone());

//     println!("Accuracy: {}/{}", correct, test_targets.len());

//     //save the nn
//     network.save("nn.json".to_string());
// }

// creating a UI for the neural network using yew.rs

use yew::prelude::*;

// creating a 28 x 28 grid where users can draw a number and the neural network will try to guess it
// loading a pretrained nn from the nn.json file

struct Model<'a> {
    value: [f64; 784],
    nn: Network<'a>,
}

enum Msg {
    Clicked(usize),
    Clear,
    Guess,
}

impl Component for Model<'static> {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut nn = Network::new(vec![784, 10, 15, 10], 0.015, SIGMOID);
        nn.load("nn.json".to_string());
        Self {
            value: [0.0; 784],
            nn,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(index) => {
                self.value[index] = 1.0;
            }
            Msg::Clear => {
                self.value = [0.0; 784];
            }
            Msg::Guess => {
                let output = self.nn.feed_forward(self.value.to_vec());
                let mut max = 0.0;
                let mut max_index = 0;
                for i in 0..output.len() {
                    if output[i] > max {
                        max = output[i];
                        max_index = i;
                    }
                }
                println!("The neural network thinks this is a {}", max_index);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid = (0..784).map(|i| {
            let color = if self.value[i] == 0.0 {
                "white"
            } else {
                "black"
            };
            html! {
                <div class="grid-item" style={format!("background-color: {}", color)} onclick={
                    ctx.link().callback(move |_| Msg::Clicked(i))
                }></div>
            }
        });
        html! {
            <div>
                <div class="grid-container">
                    { for grid }
                </div>
                <button onclick={ctx.link().callback(|_| Msg::Clear)}>{ "Clear" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Guess)}>{ "Guess" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}

fn accuracy(nn: &mut Network, test_data: Vec<Vec<f64>>, test_labels: Vec<Vec<f64>>) -> i32 {
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
    correct
}