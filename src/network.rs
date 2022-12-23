use std::{
	fs::File,
	io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

use rand::{thread_rng, seq::SliceRandom};

use super::{activations::Activation, matrix::Matrix};

pub struct Network<'a> {
	layers: Vec<usize>,
	weights: Vec<Matrix>,
	biases: Vec<Matrix>,
	data: Vec<Matrix>,
	learning_rate: f64,
	activation: Activation<'a>,
}

#[derive(Serialize, Deserialize)]
struct SaveData {
	weights: Vec<Vec<Vec<f64>>>,
	biases: Vec<Vec<Vec<f64>>>,
}

impl Network<'_> {
	pub fn new<'a>(
		layers: Vec<usize>,
		learning_rate: f64,
		activation: Activation<'a>,
	) -> Network<'a> {
		let mut weights = vec![];
		let mut biases = vec![];

		for i in 0..layers.len() - 1 {
			weights.push(Matrix::random(layers[i + 1], layers[i]));
			biases.push(Matrix::random(layers[i + 1], 1));
		}

		Network {
			layers,
			weights,
			biases,
			data: vec![],
			learning_rate,
			activation,
		}
	}

	pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
		if inputs.len() != self.layers[0] {
			panic!("Invalid inputs length");
		}

		let mut current = Matrix::from(vec![inputs]).transpose();
		self.data = vec![current.clone()];

		for i in 0..self.layers.len() - 1 {
			current = self.weights[i]
				.multiply(&current)
				.add(&self.biases[i])
				.map(self.activation.function);
			self.data.push(current.clone());
		}

		current.transpose().data[0].to_owned()
	}

	pub fn back_propogate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
		if targets.len() != self.layers[self.layers.len() - 1] {
			panic!("Invalid targets length");
		}

		let parsed = Matrix::from(vec![outputs]);
		let mut errors = Matrix::from(vec![targets]).subtract(&parsed).transpose();
		let mut gradients = parsed.map(self.activation.derivative).transpose();

		for i in (0..self.layers.len() - 1).rev() {
			gradients = gradients
				.dot_multiply(&errors)
				.map(&|x| x * self.learning_rate);

			self.weights[i] = self.weights[i].add(&gradients.multiply(&self.data[i].transpose()));
			self.biases[i] = self.biases[i].add(&gradients);

			errors = self.weights[i].transpose().multiply(&errors);
			gradients = self.data[i].map(self.activation.derivative);
		}
	}

	pub fn back_propogate_with_custom_data(&mut self, outputs: Vec<f64>, targets: Vec<f64>, data: Vec<Matrix>) {
		if targets.len() != self.layers[self.layers.len() - 1] {
			panic!("Invalid targets length");
		}

		let parsed = Matrix::from(vec![outputs]);
		let mut errors = Matrix::from(vec![targets]).subtract(&parsed).transpose();
		let mut gradients = parsed.map(self.activation.derivative).transpose();

		for i in (0..self.layers.len() - 1).rev() {
			gradients = gradients
				.dot_multiply(&errors)
				.map(&|x| x * self.learning_rate);

			self.weights[i] = self.weights[i].add(&gradients.multiply(&data[i].transpose()));
			self.biases[i] = self.biases[i].add(&gradients);

			errors = self.weights[i].transpose().multiply(&errors);
			gradients = data[i].map(self.activation.derivative);
		}
	}

	pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u16) {
		for i in 1..=epochs {
			if epochs < 100 || i % (epochs / 100) == 0 {
				println!("Epoch {} of {}", i, epochs);
			}
			for j in 0..inputs.len() {
				let outputs = self.feed_forward(inputs[j].clone());
				self.back_propogate(outputs, targets[j].clone());
			}
		}
	}

	pub fn batch_train(&mut self, training_data: Vec<(Vec<f64>, Vec<f64>)>, epochs: u16, mini_batch_size: u16) {
		for i in 1..=epochs {
			if epochs < 100 || i % (epochs / 100) == 0 {
				println!("Epoch {} of {}", i, epochs);
			}
			// shuffle training data
			let mut rng = thread_rng();
			let mut t_data = training_data.clone();
			t_data.shuffle(&mut rng);

			// split into mini batches
			let mini_batches = t_data.chunks(mini_batch_size as usize).collect::<Vec<&[(Vec<f64>, Vec<f64>)]>>();

			// train on each mini batch by averaging the errors
			for mini_batch in mini_batches {
				let mut batch_errors = vec![];
				let (inputs, targets) = mini_batch.iter().fold((vec![], vec![]), |(mut inputs, mut targets), (input, target)| {
					inputs.push(input.clone());
					targets.push(target.clone());
					(inputs, targets)
				});

				for j in 0..inputs.len() {
					let _ = self.feed_forward(inputs[j].clone());
					batch_errors.push(self.data.clone());
				}

				// avg the gradients

				// transpose the batch_errors to get layer wise errors
				let layer_wise_errors: Vec<Vec<Matrix>> = {
					let len = batch_errors[0].len();
					let mut iters: Vec<_> = batch_errors.into_iter().map(|n| n.into_iter()).collect();
					(0..len)
						.map(|_| {
							iters
								.iter_mut()
								.map(|n| n.next().unwrap())
								.collect::<Vec<Matrix>>()
						})
						.collect()
				};

				// averaging using Matrix::average(Vec<Matrix>)
				let mut layer_wise_avg_errors = vec![];
				for errors in layer_wise_errors {
					let avg_errors = Matrix::average(errors);
					layer_wise_avg_errors.push(avg_errors);
				}

				// back propogate with the averaged errors
				for j in 0..inputs.len() {
					let output = self.feed_forward(inputs[j].clone());
					self.back_propogate_with_custom_data(output, targets[j].clone(), layer_wise_avg_errors.clone());
				}
			}
		}
	}

	pub fn save(&self, file: String) {
		let mut file = File::create(file).expect("Unable to touch save file");

		file.write_all(
			json!({
				"weights": self.weights.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>(),
				"biases": self.biases.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>()
			}).to_string().as_bytes(),
		).expect("Unable to write to save file");
	}

	pub fn load(&mut self, file: String) {
		let mut file = File::open(file).expect("Unable to open save file");
		let mut buffer = String::new();

		file.read_to_string(&mut buffer)
			.expect("Unable to read save file");

		let save_data: SaveData = from_str(&buffer).expect("Unable to serialize save data");

		let mut weights = vec![];
		let mut biases = vec![];

		for i in 0..self.layers.len() - 1 {
			weights.push(Matrix::from(save_data.weights[i].clone()));
			biases.push(Matrix::from(save_data.biases[i].clone()));
		}

		self.weights = weights;
		self.biases = biases;
	}
}
