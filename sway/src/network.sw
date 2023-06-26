library network;

dep matrix;

use matrix::Matrix;
use std::logging::log;
use fixed_point::ifp64::IFP64;

pub struct Network {
    layers: Vec<u64>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    learning_rate: IFP64,
}

impl Network {
    pub fn new(layers: Vec<u64>, learning_rate: IFP64) -> Network {
        let mut weights: Vec<Matrix> = Vec::new();
        let mut biases: Vec<Matrix> = Vec::new();
        let mut i = 0;
        while i < layers.len() - 1 {
            let weight = Matrix::random(layers.get(i + 1).unwrap(), layers.get(i).unwrap());
            weights.push(weight);
            let bias = Matrix::random(layers.get(i + 1).unwrap(), 1);
            biases.push(bias);
            i += 1;
        }
        Network {
            layers,
            weights,
            biases,
            data: Vec::new(),
            learning_rate,
        }
    }

    pub fn feed_forward(ref mut self, inputs: Vec<IFP64>) -> Vec<IFP64> {
        if inputs.len() != self.layers.get(0).unwrap() {
            log("Invalid inputs length");
            revert(0);
        }

        let mut current: Vec<Vec<IFP64>> = Vec::new();
        current.push(inputs);
        let mut current = Matrix::from(current).transpose();

        let mut data: Vec<Matrix> = Vec::new();
        data.push(current);
        self.data = data;

        let mut i = 0;
        while i < self.layers.len() - 1 {
            current = self.weights.get(i).unwrap().multiply(current).add(self.biases.get(i).unwrap()).sigmoid_every_element();
            self.data.push(current);
            i += 1;
        }
        current.transpose().data.get(0).unwrap()
    }

    pub fn back_propogate(ref mut self, outputs: Vec<IFP64>, targets: Vec<IFP64>) {
        if targets.len() != self.layers.get(self.layers.len() - 1).unwrap()
        {
            log("Invalid targets length");
            revert(0);
        }

        let mut parsed: Vec<Vec<IFP64>> = Vec::new();
        parsed.push(outputs);
        let parsed = Matrix::from(parsed);

        let mut errors: Vec<Vec<IFP64>> = Vec::new();
        errors.push(targets);
        let mut errors = Matrix::from(errors).subtract(parsed).transpose();

        let mut gradients = parsed.sigmoid_derivative_every_element().transpose();

        let mut i = self.layers.len() - 1;
        while i > 0 {
            gradients = gradients.dot_multiply(errors).multiply_every_element(self.learning_rate);

            self.weights.set(i, self.weights.get(i).unwrap().add(gradients.multiply(self.data.get(i).unwrap().transpose())));
            self.biases.set(i, self.biases.get(i).unwrap().add(gradients));

            errors = self.weights.get(i).unwrap().transpose().multiply(errors);
            gradients = self.data.get(i).unwrap().sigmoid_derivative_every_element();

            i -= 1;
        }
    }
}
