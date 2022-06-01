use borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{self, Serialize, Deserialize};

extern crate rand;
use rand::prelude::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Default, Clone, Copy)]
pub struct SimpleStruct {
    pub a: i32,
    pub b: i32,
}

impl SimpleStruct {

    pub fn default() -> Self {
        Self {
            a: 0, b: 0
        }
    }

    pub fn random(&self) -> SimpleStruct {
        let mut rng = thread_rng();
        SimpleStruct {
            a: rng.gen(),
            b: rng.gen(),
        }
    }

    pub fn new(a: i32, b: i32) -> SimpleStruct {
        SimpleStruct { a, b }
    }

    pub fn add(&self, other: &SimpleStruct) -> SimpleStruct {
        SimpleStruct {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }

    pub fn subtract(&self, other: &SimpleStruct) -> SimpleStruct {
        SimpleStruct {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
