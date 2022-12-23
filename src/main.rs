use activations::SIGMOID;
use network::Network;
use rust_mnist::Mnist;

pub mod activations;
pub mod matrix;
pub mod network;

// fn main() {
// 	let inputs = vec![
// 		vec![0.0, 0.0],
// 		vec![0.0, 1.0],
// 		vec![1.0, 0.0],
// 		vec![1.0, 1.0]
// 	];
// 	let targets = vec![
// 		vec![1.0, 0.0],
// 		vec![0.0, 1.0],
// 		vec![0.0, 1.0],
// 		vec![1.0, 0.0]
// 	];
	
// 	let mut network = Network::new(vec![2, 3, 2], 0.5, SIGMOID);
	
// 	network.train(inputs, targets, 1000);

// 	println!("{:?}", network.feed_forward(vec![0.0, 0.0]));
// 	println!("{:?}", network.feed_forward(vec![0.0, 1.0]));
// 	println!("{:?}", network.feed_forward(vec![1.0, 0.0]));
// 	println!("{:?}", network.feed_forward(vec![1.0, 1.0]));
// }


use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

// std imports for time measurement
use std::time;

// pub mod nn;
// pub mod harness;

// use nn::{activations::SIGMOID, network::{loss, Network},};

// // Makes a dataset of random points in a circle with radius r, and returns a vector of tuples of the form (vector of x and y coordinates, 0 or 1 depending on whether the point is inside the circle or not)
// pub fn circle_dataset(r: f64, num_samples: u64) -> Vec<(Vec<f64>, Vec<f64>)> {
//     let mut rng = thread_rng();
//     let range = Uniform::new(r * -2.0, r * 2.0);

//     let random_samples: Vec<Vec<f64>> = (0..num_samples).map(|_| vec![rng.sample(range), rng.sample(range)]).collect();

//     let output_samples: Vec<(Vec<f64>, Vec<f64>)> = random_samples.iter().map(|vec| (vec.to_owned(), vec![(vec[0].powf(2.0) + vec[1].powf(2.0) <= r.powf(2.0)) as usize as f64])).collect();
//     output_samples
// }

// The above function but with the targets as a one hot encoded vector
pub fn circle_dataset(r: f64, num_samples: u64) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let mut rng = thread_rng();
    let range = Uniform::new(r * -2.0, r * 2.0);

    let random_samples: Vec<Vec<f64>> = (0..num_samples).map(|_| vec![rng.sample(range), rng.sample(range)]).collect();

    let inside_circle = [1.0, 0.0];
    let outside_circle = [0.0, 1.0];

    let output_samples: Vec<(Vec<f64>, Vec<f64>)> = random_samples.iter().map(|vec| (vec.to_owned(), if vec[0].powf(2.0) + vec[1].powf(2.0) <= r.powf(2.0) { inside_circle.to_vec() } else { outside_circle.to_vec() })).collect();
    let (inputs, targets): (Vec<Vec<f64>>, Vec<Vec<f64>>) = output_samples.into_iter().unzip();
    (inputs, targets)
}

// OCR MNIST dataset
// pub fn mnist_dataset() -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
//     // obtain the images and labels from the mnist dataset


fn main() {
    let mnist = Mnist::new(r"data\");
    let inputs: Vec<Vec<f64>> = mnist.train_data.iter().map(|x| x.iter().map(|y| *y as f64 / 256.0).collect()).collect();
    let targets: Vec<Vec<f64>> = mnist.train_labels.iter().map(|x| {
        let mut vec = vec![0.0; 10];
        vec[*x as usize] = 1.0;
        vec
    }).collect();

    // zip the two vectors together to make a vector of tuples
    let training_data: Vec<(Vec<f64>, Vec<f64>)> = inputs.iter().zip(targets.iter()).map(|(x, y)| (x.to_owned(), y.to_owned())).collect();

    let test_inputs: Vec<Vec<f64>> = mnist.test_data.iter().map(|x| x.iter().map(|y| *y as f64 / 256.0).collect()).collect();
    let test_targets: Vec<Vec<f64>> = mnist.test_labels.iter().map(|x| {
        let mut vec = vec![0.0; 10];
        vec[*x as usize] = 1.0;
        vec
    }).collect();

    // zip the two vectors together to make a vector of tuples
    // let test_data: Vec<(Vec<f64>, Vec<f64>)> = test_inputs.iter().zip(test_targets.iter()).map(|(x, y)| (x.to_owned(), y.to_owned())).collect();

    let mut network = Network::new(vec![784, 100, 100, 10], 0.005, SIGMOID);
    // network.load("nn.json".to_string());

    // print the accuracy before trainig
    println!("Loss before training: {}", loss(&mut network, test_inputs.to_owned(), test_targets.to_owned()));
    let mut correct = 0;
    for i in 0..test_inputs.len() {
        let output = network.feed_forward(test_inputs[i].to_owned());
        let mut max = 0.0;
        let mut max_index = 0;
        for j in 0..output.len() {
            if output[j] > max {
                max = output[j];
                max_index = j;
            }
        }
        if test_targets[i][max_index] == 1.0 {
            correct += 1;
        }
    }

    println!("Accuracy: {}/{}", correct, test_inputs.len());

    // time before training
    let start = time::Instant::now();
    network.train(inputs.clone(), targets, 10);
    // time after training
    let end = time::Instant::now();
    println!("Time taken: {}s", (end - start).as_secs());
    // network.batch_train(training_data, 10, 10);

    // print the accuracy after training    
    println!("Loss after training: {}", loss(&mut network, test_inputs.to_owned(), test_targets.to_owned()));

    //count how many test samples are correctly classified
    let mut correct = 0;
    for i in 0..test_inputs.len() {
        let output = network.feed_forward(test_inputs[i].to_owned());
        let mut max = 0.0;
        let mut max_index = 0;
        for j in 0..output.len() {
            if output[j] > max {
                max = output[j];
                max_index = j;
            }
        }
        if test_targets[i][max_index] == 1.0 {
            correct += 1;
        }
    }

    println!("Accuracy: {}/{}", correct, test_inputs.len());

    //save the nn
    network.save("nn.json".to_string());
}

pub fn loss(network: &mut Network, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>) -> f64 {
	let mut loss = 0.0;
	for i in 0..inputs.len() {
		let output = network.feed_forward(inputs[i].to_owned());
		for j in 0..output.len() {
			let clipped = output[j].max(0.000000001).min(0.999999999);
			loss += targets[i][j] * clipped.ln();
		}
	}
	-loss / inputs.len() as f64
}