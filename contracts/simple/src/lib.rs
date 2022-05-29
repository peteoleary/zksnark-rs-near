extern crate near_sdk;
use self::near_sdk::{metadata, near_bindgen, PanicOnDefault};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
struct SimpleContract {
    nothing: u16
}

impl Default for SimpleContract {
    fn default () -> Self {
        Self {
            nothing: 0
        }
    }
}

impl SimpleContract {

    pub fn get_nothing(&self) -> u16 {
        return self.nothing
    }

    pub fn get_something(&self, add: u16) -> u16 {
        return self.nothing + add;
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


    #[test]
    fn setup_test() {
        let context = get_context(false);
        testing_env!(context);
        let contract = SimpleContract::default();
        assert_eq!(
            contract.get_nothing(),
            0
        );
        assert_eq!(
            contract.get_something(10),
            10
        );
    }

}
