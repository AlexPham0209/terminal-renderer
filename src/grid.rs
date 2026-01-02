use core::fmt;
use std::clone::Clone;
use std::string::ToString;

pub struct Grid<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index: usize = y * self.width + x;
        self.data.get(index)
    }

    pub fn set(&mut self, value: T, x: usize, y: usize) -> bool {
        let index: usize = y * self.width + x;
        if index >= self.data.len() || y >= self.height || x >= self.width {
            return false;
        }

        self.data[index] = value;
        true
    }

    pub fn clear(&mut self, value: T)
    where
        T: Copy,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(value, x, y);
            }
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(fill_value: T, width: usize, height: usize) -> Self {
        Self {
            data: vec![fill_value; width * height],
            width,
            height,
        }
    }
}

impl<T: ToString> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: String = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(value) = self.get(x, y) {
                    res.push_str(&value.to_string());
                }
            }
            res.push_str("\n");
        }

        write!(f, "{}", res)
    }
}
