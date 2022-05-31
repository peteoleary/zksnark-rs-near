extern crate near_sdk;
use near_sdk::{metadata, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{self, Deserialize, Serialize};

use zksnark::groth16::fr::FrLocal;
use zksnark::setup_file::{SetupFile, CHECK};
use zksnark::proof_file::{ProofFile};

#[derive(BorshDeserialize, BorshSerialize, Debug, Default, Serialize, Deserialize)]
#[near_bindgen]
struct SetupContract {
    setup_file: SetupFile
}

#[near_bindgen]
impl SetupContract {

    pub fn default () -> Self {
        Self {
            setup_file: SetupFile::default()
        }
    }

    pub fn verify(&self, assignments: &Box<[FrLocal]>, proof: ProofFile) -> bool {
        return self.setup_file.verify(assignments, proof)
    }

    pub fn from_zk(code: &String) -> Self {
        Self {
            setup_file: SetupFile::from_zk(code)
        }        
    }

    pub fn from_setup(setup_file: SetupFile) -> Self {
        Self {
            setup_file: setup_file
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
        let contract = SetupContract::from_zk(TEST_ZK);
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

        assert!(setup_contract.verify(&output_assignments(), proof_file))
    }

    use workspaces::prelude::*;

    #[tokio::test]
    async fn test_cross_contract() -> anyhow::Result<()> {
        let worker = workspaces::sandbox().await?;
        let contract = worker.dev_deploy(&std::fs::read(format!("../res/setup.wasm"))?).await?;

        let res = contract
            .call(&worker, "from_zk")
            .args_json(TEST_ZK)?
            .gas(300_000_000_000_000)
            .transact()
            .await?;
        assert!(res.is_success());

        Ok(())
    }

}
