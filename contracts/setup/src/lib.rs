extern crate near_sdk;
use self::near_sdk::{metadata, near_bindgen, PanicOnDefault};

use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

use zksnark::groth16::fr::FrLocal;
use zksnark::setup_file::{SetupFile, CHECK};
use zksnark::proof_file::{ProofFile};

#[derive(BorshDeserialize, BorshSerialize, Debug, PanicOnDefault)]
#[near_bindgen]
struct SetupContract {
    setup_file: SetupFile
}

impl SetupContract {
    pub fn verify(&self, assignments: &[FrLocal], proof: ProofFile) -> bool {
        return self.setup_file.verify(assignments, proof)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use proof::{ProofContract};
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

    #[test]
    fn setup_verify_test() {
        let context = get_context(false);
        testing_env!(context);
        let setup_file = SetupFile::from_zk(TEST_ZK);
        let setup_contract = SetupContract { setup_file: setup_file.clone()};
        
        let proof_file =  ProofFile::from_setup(&input_assignments(), setup_file);
        // let proof_contract = ProofContract {proof_file: proof_file};


        assert!(setup_contract.verify(&output_assignments(), proof_file))
    }
}
