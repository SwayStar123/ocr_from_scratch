contract;

dep matrix;
dep activations;

// use matrix::Matrix;

abi MyContract {
    fn test_function() -> bool;
}

impl MyContract for Contract {
    fn test_function() -> bool {
        true
    }
}
