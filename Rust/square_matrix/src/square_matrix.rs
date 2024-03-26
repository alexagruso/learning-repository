use std::{
    fmt::Display,
    ops::{Add, Mul},
};

#[allow(dead_code)]
pub struct SquareMatrix<T> {
    size: usize,
    data: Vec<Vec<Option<T>>>,
}

impl<T: Add + Mul + Clone + Display> SquareMatrix<T> {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Matrix size must be at least 1, got {}", size);
        }

        let mut data: Vec<Vec<Option<T>>> = Vec::new();
        data.reserve(size);

        for _ in 0..size {
            let mut row: Vec<Option<T>> = Vec::new();
            row.resize(size, None);

            data.push(row);
        }

        SquareMatrix { size, data }
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if let Some(row) = self.data.get(row) {
            if let Some(entry) = row.get(column) {
                return entry.as_ref();
            }
        }

        None
    }

    pub fn set(&mut self, row: usize, column: usize, value: T) {
        if let Some(row) = self.data.get_mut(row) {
            if let Some(entry) = row.get_mut(column) {
                *entry = Some(value.to_owned());
            }
        }
    }

    pub fn set_all_to(&mut self, value: T) {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                *entry = Some(value.to_owned());
            }
        }
    }

    pub fn print(&self) {
        for row in self.data.iter() {
            for entry in row.iter() {
                if let Some(value) = entry {
                    print!("{} ", value);
                }
            }

            println!();
        }
    }
}
