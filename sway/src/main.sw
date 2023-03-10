contract;

dep network;
dep utils;

use network::Network;
use std::storage::StorageVec;
use sway_libs::ufp64::UFP64;
use utils::StorageMatrixVec;

abi MyContract {
    #[storage(read, write)]
    fn init_network(layers: Vec<u64>, learning_rate: UFP64);
    #[storage(read)]
    fn feed_forward(input: Vec<UFP64>) -> Vec<UFP64>;
    #[storage(read, write)]
    fn back_propogate(input: Vec<UFP64>, expected: Vec<UFP64>);
}

storage {
    layers: StorageVec<u64> = StorageVec {},
    weights: StorageMatrixVec = StorageMatrixVec {},
    biases: StorageMatrixVec = StorageMatrixVec {},
    learning_rate: UFP64 = UFP64 { value: 0 },
}

impl MyContract for Contract {
    #[storage(read, write)]
    fn init_network(layers: Vec<u64>, learning_rate: UFP64) {
        storage.learning_rate = learning_rate;
        let network = Network::new(layers, learning_rate);
        storage.weights.from(network.weights);
        storage.biases.from(network.biases);

        let mut i = 0;
        while i < layers.len() {
            storage.layers.push(layers.get(i).unwrap());
            i += 1;
        }
    }

    #[storage(read)]
    fn feed_forward(input: Vec<UFP64>) -> Vec<UFP64> {
        let mut layers: Vec<u64> = Vec::new();
        let mut i = 0;
        while i < storage.layers.len() {
            layers.push(storage.layers.get(i).unwrap());
            i += 1;
        }
        let mut network = Network { layers: layers, weights: storage.weights.to(), biases: storage.biases.to(), learning_rate: storage.learning_rate, data: Vec::new()};

        network.feed_forward(input)
    }

    #[storage(read, write)]
    fn back_propogate(input: Vec<UFP64>, expected: Vec<UFP64>) {
        let mut layers: Vec<u64> = Vec::new();
        let mut i = 0;
        while i < storage.layers.len() {
            layers.push(storage.layers.get(i).unwrap());
            i += 1;
        }
        let mut network = Network { layers: layers, weights: storage.weights.to(), biases: storage.biases.to(), learning_rate: storage.learning_rate, data: Vec::new()};

        network.back_propogate(input, expected);

        storage.weights.from(network.weights);
        storage.biases.from(network.biases);
    }
}
