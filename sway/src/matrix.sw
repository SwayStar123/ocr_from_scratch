library;

use fixed_point::ifp64::IFP64;
use std::logging::log;
use activations::{sigmoid, sigmoid_derivative};

pub fn zeroes_vec(ref mut vec: Vec<IFP64>, len: u64) {
    let mut i = 0;
    while i < len {
        vec.push(IFP64::zero());
        i += 1;
    }
}

pub struct Matrix {
    rows: u64,
    cols: u64,
    data: Vec<Vec<IFP64>>,
}

pub fn zeroes(rows: u64, cols: u64) -> Matrix {
    let mut data: Vec<Vec<IFP64>> = Vec::with_capacity(rows);
    let mut i = 0;
    while i < rows {
        let mut row: Vec<IFP64> = Vec::with_capacity(cols);
        zeroes_vec(row, cols);
        data.push(row);
        i += 1;
    }

    Matrix {
        rows,
        cols,
        data,
    }
}

impl Matrix {
    pub fn new(rows: u64, cols: u64, data: Vec<Vec<IFP64>>) -> Matrix {
        Matrix {
            rows,
            cols,
            data,
        }
    }

    // pub fn zeroes(rows: u64, cols: u64) -> Matrix {
    //     let mut data = Vec::with_capacity(rows);
    //     let mut i = 0;
    //     while i < rows {
    //         let mut row = Vec::with_capacity(cols);
    //         zeroes_vec(row, cols);
    //         data.push(row);
    //         i += 1;
    //     }
    //     Matrix {
    //         rows,
    //         cols,
    //         data,
    //     }
    // }
    pub fn from(data: Vec<Vec<IFP64>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data.get(0).unwrap().len(),
            data,
        }
    }
}

impl Matrix {
    pub fn multiply(self, other: Matrix) -> Matrix {
        if self.cols != other.rows {
            log(("self.rows, self.cols = ", self.rows, self.cols));
            log(("other.rows, other.cols = ", other.rows, other.cols));
            log("Attempted to multiply matrix of incorrect dimensions");
            revert(0);
        }

        let mut res = zeroes(self.rows, other.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < other.cols {
                let mut k = 0;
                while k < self.cols {
                    res.data.get(i).unwrap().set(j, res.data.get(i).unwrap().get(j).unwrap() + self.data.get(i).unwrap().get(k).unwrap() * other.data.get(k).unwrap().get(j).unwrap());
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn add(self, other: Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            log(("self.rows, self.cols = ", self.rows, self.cols));
            log(("other.rows, other.cols = ", other.rows, other.cols));
            log("Attempted to add matrix of incorrect dimensions");
            revert(0);
        }

        let mut res = zeroes(self.rows, self.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(i).unwrap().set(j, self.data.get(i).unwrap().get(j).unwrap() + other.data.get(i).unwrap().get(j).unwrap());
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn subtract(self, other: Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            log(("self.rows, self.cols = ", self.rows, self.cols));
            log(("other.rows, other.cols = ", other.rows, other.cols));
            log("Attempted to subtract matrix of incorrect dimensions");
            revert(0);
        }

        let mut res = zeroes(self.rows, self.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(i).unwrap().set(j, self.data.get(i).unwrap().get(j).unwrap() - other.data.get(i).unwrap().get(j).unwrap());
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn transpose(self) -> Matrix {
        let mut res = zeroes(self.cols, self.rows);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(j).unwrap().set(i, self.data.get(i).unwrap().get(j).unwrap());
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn sigmoid_every_element(self) -> Matrix {
        let mut res = zeroes(self.rows, self.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(i).unwrap().set(j, sigmoid(self.data.get(i).unwrap().get(j).unwrap()));
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn sigmoid_derivative_every_element(self) -> Matrix {
        let mut res = zeroes(self.rows, self.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(i).unwrap().set(j, sigmoid_derivative(self.data.get(i).unwrap().get(j).unwrap()));
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn multiply_every_element(self, other: IFP64) -> Matrix {
        let mut res = zeroes(self.rows, self.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(i).unwrap().set(j, self.data.get(i).unwrap().get(j).unwrap() * other);
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn random(rows: u64, cols: u64) -> Matrix {
        let mut res = zeroes(rows, cols);

        res
    }

    pub fn dot_multiply(self, other: Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            log(("self.rows, self.cols = ", self.rows, self.cols));
            log(("other.rows, other.cols = ", other.rows, other.cols));
            log("Attempted to dot multiply matrix of incorrect dimensions");
            revert(0);
        }

        let mut res = zeroes(self.rows, self.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(i).unwrap().set(j, self.data.get(i).unwrap().get(j).unwrap() * other.data.get(i).unwrap().get(j).unwrap());
                j += 1;
            }
            i += 1;
        }

        res
    }
}
