library activations;

use sway_libs::ufp64::UFP64;

pub fn sigmoid(x: UFP64) -> UFP64 {
    // should be
    // UFP64::from_uint(1) / (UFP64::from_uint(1) + (-x).exp())
    UFP64::from_uint(1) / (UFP64::from_uint(1) + UFP64::exp(x))
}

pub fn sigmoid_derivative(x: UFP64) -> UFP64 {
    x * (UFP64::from_uint(1) - x)
}
