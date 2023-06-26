library;

use ::matrix::Matrix;
use std::{hash::sha256, storage::{get, store}};
use fixed_point::ifp64::IFP64;

// for storing Vec<Matrix> in storage
pub struct StorageMatrixVec {}

impl StorageMatrixVec {
    #[storage(write)]
    pub fn from(self, matrix_vec: Vec<Matrix>) {
        let mut matrix_id = 0;
        while matrix_id < matrix_vec.len() {
            let mut i = 0;
            while i < matrix_vec.get(matrix_id).unwrap().rows {
                let mut j = 0;
                while j < matrix_vec.get(matrix_id).unwrap().cols {
                    let hash = sha256((__get_storage_key(), matrix_id, i, j));
                    store(hash, matrix_vec.get(matrix_id).unwrap().data.get(i).unwrap().get(j).unwrap());
                    j += 1;
                }
                i += 1;
            }

            let col_hash = sha256((__get_storage_key(), matrix_id, "cols"));
            let row_hash = sha256((__get_storage_key(), matrix_id, "rows"));

            store(col_hash, matrix_vec.get(matrix_id).unwrap().cols);
            store(row_hash, matrix_vec.get(matrix_id).unwrap().rows);
            matrix_id += 1;
        }

        let hash = sha256((__get_storage_key()));
        store(hash, matrix_vec.len());
    }

    #[storage(read)]
    pub fn to(self) -> Vec<Matrix> {
        let len_hash = sha256((__get_storage_key()));
        let len = get::<u64>(len_hash);

        let mut matrix_vec = Vec::new();
        let mut matrix_id = 0;

        // get the cols and rows beforehand and make new vecs using Vec::with_capacity to avoid resizing
        while matrix_id < len {
            let col_hash = sha256((__get_storage_key(), matrix_id, "cols"));
            let row_hash = sha256((__get_storage_key(), matrix_id, "rows"));

            let cols = get::<u64>(col_hash);
            let rows = get::<u64>(row_hash);

            let mut i = 0;
            let mut data: Vec<Vec<IFP64>> = Vec::with_capacity(rows);
            while i < rows {
                let mut j = 0;
                let mut row: Vec<IFP64> = Vec::with_capacity(cols);
                while j < cols {
                    let hash = sha256((__get_storage_key(), matrix_id, i, j));
                    row.push(get::<IFP64>(hash));
                    j += 1;
                }
                data.push(row);
                i += 1;
            }
        }

        matrix_vec
    }
}
