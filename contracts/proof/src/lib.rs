extern crate near_sdk;
use self::near_sdk::{env, log, metadata, near_bindgen, AccountId};

use borsh::{BorshSerialize, BorshDeserialize};

use zksnark::proof_file::{ProofFile};
use zksnark::setup_file::{SetupFile, CHECK};
use zksnark::FrLocal;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
pub struct ProofContract {
    proof_file: ProofFile
}

#[near_bindgen]
impl ProofContract {
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn input_assignments() -> [FrLocal; 3] {
        return [
            FrLocal::from(3), // a
            FrLocal::from(2), // b
            FrLocal::from(4) // c
        ];
    } 

    fn output_assignments() -> [FrLocal; 2] {
        return [
            FrLocal::from(2),
            FrLocal::from(34)
        ];
    } 

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
    fn setup_proof_from_file_test() {
        let context = get_context(false);
        testing_env!(context);
        let setup_file = SetupFile::from_zk(TEST_ZK);
        let proof_contract = ProofContract {proof_file: ProofFile::from_setup(&input_assignments(), setup_file)};
        assert_eq!(
            proof_contract.proof_file.check,
            CHECK
        );
    }
}
