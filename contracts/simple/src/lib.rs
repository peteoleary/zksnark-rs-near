extern crate near_sdk;
use near_sdk::{metadata, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

extern crate simple_lib;

use self::simple_lib::SimpleStruct;

// extern crate rabe_bn as bn;
// use self::bn::{G1};

type Nothing = SimpleStruct;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct SimpleContract {
    nothing: Nothing
}

impl Default for SimpleContract {
    fn default () -> Self {
        Self {
            
            nothing: Nothing::default()
        }
    }
}

#[near_bindgen]
impl SimpleContract {

    pub fn get_nothing(&self) -> Nothing {
        return self.nothing
    }

    pub fn get_random(&self) -> Nothing {
        return self.nothing.random()
    }

    pub fn get_something(&self, something: Nothing) -> Nothing {
        return self.nothing.add(&something);
    }

}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

}
