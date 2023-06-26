library activations;

use fixed_point::ifp64::IFP64;

pub fn sigmoid(x: IFP64) -> IFP64 {
    // should be
    IFP64::from_uint(1) / (IFP64::from_uint(1) + x.sign_reverse().exp())
    // IFP64::from_uint(1) / (IFP64::from_uint(1) + IFP64::exp(x))
}

pub fn sigmoid_derivative(x: IFP64) -> IFP64 {
    x * (IFP64::from_uint(1) - x)
}
