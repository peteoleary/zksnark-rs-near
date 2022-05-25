extern crate near_sdk;
use self::near_sdk::{metadata, near_bindgen};

use borsh::{BorshSerialize, BorshDeserialize};

use zksnark::setup_file::{SetupFile, CHECK};
use zksnark::proof_file::{ProofFile};

metadata! {
#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
struct SetupContract {
    setup_file: SetupFile
}

#[near_bindgen]
impl SetupContract {
    pub fn verify(proof_file: ProofFile) {

    }
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

    const TEST_ZK: &str = "
        (in a b c)
        (out x)
        (verify b x)

        (program
            (= temp
                (* a b))
            (= x
                (* 1 (+ (* 4 temp) c 6))))
    ";

    #[test]
    fn setup_test() {
        let context = get_context(false);
        testing_env!(context);
        let contract = SetupContract { setup_file: SetupFile::from_zk(TEST_ZK)};
        assert_eq!(
            contract.setup_file.check,
            CHECK
        );
    }
}
