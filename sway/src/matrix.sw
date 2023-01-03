// use rand::{thread_rng, Rng};
// use std::fmt::{Debug, Formatter, Result};

// #[derive(Clone)]
// pub struct Matrix {
// 	pub rows: usize,
// 	pub cols: usize,
// 	pub data: Vec<Vec<f64>>,
// }

// impl Matrix {
// 	pub fn zeros(rows: usize, cols: usize) -> Matrix {
// 		Matrix {
// 			rows,
// 			cols,
// 			data: vec![vec![0.0; cols]; rows],
// 		}
// 	}

// 	pub fn random(rows: usize, cols: usize) -> Matrix {
// 		let mut rng = thread_rng();

// 		let mut res = Matrix::zeros(rows, cols);
// 		for i in 0..rows {
// 			for j in 0..cols {
// 				res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
// 			}
// 		}

// 		res
// 	}

// 	pub fn from(data: Vec<Vec<f64>>) -> Matrix {
// 		Matrix {
// 			rows: data.len(),
// 			cols: data[0].len(),
// 			data,
// 		}
// 	}

// 	pub fn multiply(&self, other: &Matrix) -> Matrix {
// 		if self.cols != other.rows {
// 			panic!("Attempted to multiply by matrix of incorrect dimensions. self = {} x {}, other = {} x {}", self.rows, self.cols, other.rows, other.cols);
// 		}

// 		let mut res = Matrix::zeros(self.rows, other.cols);

// 		for i in 0..self.rows {
// 			for j in 0..other.cols {
// 				let mut sum = 0.0;
// 				for k in 0..self.cols {
// 					sum += self.data[i][k] * other.data[k][j];
// 				}

// 				res.data[i][j] = sum;
// 			}
// 		}

// 		res
// 	}

// 	pub fn add(&self, other: &Matrix) -> Matrix {
// 		if self.rows != other.rows || self.cols != other.cols {
// 			panic!("Attempted to add matrix of incorrect dimensions. self = {} x {}, other = {} x {}", self.rows, self.cols, other.rows, other.cols);
// 		}

// 		let mut res = Matrix::zeros(self.rows, self.cols);

// 		for i in 0..self.rows {
// 			for j in 0..self.cols {
// 				res.data[i][j] = self.data[i][j] + other.data[i][j];
// 			}
// 		}

// 		res
// 	}

// 	pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
// 		if self.rows != other.rows || self.cols != other.cols {
// 			panic!("Attempted to dot multiply by matrix of incorrect dimensions. self = {} x {}, other = {} x {}", self.rows, self.cols, other.rows, other.cols);
// 		}

// 		let mut res = Matrix::zeros(self.rows, self.cols);

// 		for i in 0..self.rows {
// 			for j in 0..self.cols {
// 				res.data[i][j] = self.data[i][j] * other.data[i][j];
// 			}
// 		}

// 		res
// 	}

// 	pub fn subtract(&self, other: &Matrix) -> Matrix {
// 		if self.rows != other.rows || self.cols != other.cols {
// 			panic!("Attempted to subtract matrix of incorrect dimensions. self = {} x {}, other = {} x {}", self.rows, self.cols, other.rows, other.cols);
// 		}

// 		let mut res = Matrix::zeros(self.rows, self.cols);

// 		for i in 0..self.rows {
// 			for j in 0..self.cols {
// 				res.data[i][j] = self.data[i][j] - other.data[i][j];
// 			}
// 		}

// 		res
// 	}

// 	pub fn map(&self, function: &dyn Fn(f64) -> f64) -> Matrix {
// 		Matrix::from(
// 			(self.data)
// 				.clone()
// 				.into_iter()
// 				.map(|row| row.into_iter().map(|value| function(value)).collect())
// 				.collect(),
// 		)
// 	}

// 	pub fn transpose(&self) -> Matrix {
// 		let mut res = Matrix::zeros(self.cols, self.rows);

// 		for i in 0..self.rows {
// 			for j in 0..self.cols {
// 				res.data[j][i] = self.data[i][j];
// 			}
// 		}

// 		res
// 	}

// 	pub fn average(matrixes: Vec<Matrix>) -> Matrix {
// 		let len = matrixes.len();
// 		let mut res = Matrix::zeros(matrixes[0].rows, matrixes[0].cols);

// 		for matrix in matrixes {
// 			res = res.add(&matrix);
// 		}

// 		let mut res = res.data;
// 		for i in 0..res.len() {
// 			for j in 0..res[0].len() {
// 				res[i][j] /= len as f64;
// 			}
// 		}

// 		Matrix::from(res)
// 	}
// }

// impl Debug for Matrix {
// 	fn fmt(&self, f: &mut Formatter) -> Result {
// 		write!(
// 			f,
// 			"Matrix {{\n{}\n}}",
// 			(&self.data)
// 				.into_iter()
// 				.map(|row| "  ".to_string()
// 					+ &row
// 						.into_iter()
// 						.map(|value| value.to_string())
// 						.collect::<Vec<String>>()
// 						.join(" "))
// 				.collect::<Vec<String>>()
// 				.join("\n")
// 		)
// 	}
// }

// the above code is in the rust language. I am trying to rewrite it in Sway, which is very similar to rust

library matrix;

use sway_libs::fixed_point::ufp::ufp64::UFP64;

pub fn zeroes_vec(ref mut vec: Vec<UFP64>, len: u64) {
    let mut i = 0;
    while i < len {
        vec.push(UFP64::zero());
        i += 1;
    }
}

pub struct Matrix {
    rows: u64,
    cols: u64,
    // temporarily in UFP64 as f64 is not supported yet
    data: Vec<Vec<UFP64>>,
}

impl Matrix {
    pub fn new(rows: u64, cols: u64, data: Vec<Vec<UFP64>>) -> Matrix {
        Matrix {
            rows,
            cols,
            data,
        }
    }

    pub fn zeros(rows: u64, cols: u64) -> Matrix {
        let mut data = Vec::with_capacity(rows);
        let mut i = 0;
        while i < rows {
            let mut row = Vec::with_capacity(cols);
            zeroes_vec(ref row, cols);
            data.push(row);
            i += 1;
        }

        Matrix {
            rows,
            cols,
            data,
        }
    }

    pub fn from(data: Vec<Vec<UFP64>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data.get(0).unwrap().len(),
            data,
        }
    }

    pub fn multiply(self, other: Matrix) -> Matrix {
        if self.cols != other.rows {
            revert("Attempted to multiply matrix of incorrect dimensions");
            log(("self.rows, self.cols = ", self.rows, self.cols));
            log(("other.rows, other.cols = ", other.rows, other.cols));
        }

        let mut res = Matrix::zeros(self.rows, other.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < other.cols {
                let mut k = 0;
                while k < self.cols {
                    res.data.get(i).set(j, res.data.get(i).get(j) + self.data.get(i).get(k) * other.data.get(k).get(j));
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
            revert("Attempted to add matrix of incorrect dimensions");
            log(("self.rows, self.cols = ", self.rows, self.cols));
            log(("other.rows, other.cols = ", other.rows, other.cols));
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(i).set(j, self.data.get(i).get(j) + other.data.get(i).get(j));
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn subtract(self, other: Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            revert("Attempted to subtract matrix of incorrect dimensions");
            log(("self.rows, self.cols = ", self.rows, self.cols));
            log(("other.rows, other.cols = ", other.rows, other.cols));
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(i).set(j, self.data.get(i).get(j) - other.data.get(i).get(j));
                j += 1;
            }
            i += 1;
        }

        res
    }

    pub fn transpose(self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);

        let mut i = 0;
        while i < self.rows {
            let mut j = 0;
            while j < self.cols {
                res.data.get(j).set(i, self.data.get(i).get(j));
                j += 1;
            }
            i += 1;
        }

        res
    }
}