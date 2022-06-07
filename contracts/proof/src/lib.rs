extern crate near_sdk;
use near_sdk::{metadata, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{self, Deserialize, Serialize};

use zksnark::proof_file::{ProofFile};
use zksnark::setup_file::{SetupFile, CHECK, Fileish};
use zksnark::FrLocal;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Default, Clone)]
#[near_bindgen]
pub struct ProofContract {
    proof_file: ProofFile
}

#[near_bindgen]
impl ProofContract {

    pub fn default () -> Self {
        Self {
            proof_file: ProofFile::default()
        }
    }

    // NOTE: there is no method to create the proof file on chain because doing so would reveal the input data
    // use the command line tool to create the proof file

    pub fn from_hex_string(&mut self, hex: &String) -> Self {
        self.proof_file = ProofFile::from_hex_string(hex.to_string());
        self.clone()
    }

    pub fn to_hex_string(&self) -> String {
        self.proof_file.to_hex_string()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn input_assignments() -> [FrLocal; 3] {
        return [
            FrLocal::from(3), // a
            FrLocal::from(2), // b
            FrLocal::from(4) // c
        ];
    } 

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    const TEST_HEX: &str = "ea1dadabb80b510fd9924918b01f4fedd7daa26b24de3f91eef16ab2b13b4437600ece08945e9194ccc4734a7b2bd37b9cf30a397ebaa6fc52fa3c15475388481476ed07355c99a765835eb596f64c7665530cf764f008e92d855bd873bef7058e596008769cc5021db04522ef5a90103f95c4cb7cecbd67b18cd891a28438704abf7c2918c0a50497b624fa83a66b3e9b68e237eefd9d63c1307b9c0b1f261ea59ee603110b35a9d0b21096ba858b062e19ce7ab3760057f6bf21ff9060468eb23efe0be4c9363e708455fe9be0720425ff6db64fa68b20b696209f63dfd7f684e2ec116d1c2d8528fd7d5d450bdf104a25923d46784a3c681ff1f84f4588a4a1e2df29ae9fcd0de1c2b6e77e5be66ac294f0158b0675b7334acc6ffd63f043112bfa1feb826dfa7981a18d05b1a390affd5a95090ce68acf35ef90859b9f41f787ce2eeae726d309750ffa776001340299aecdeb3dba59ad3e29d2da1fb0e3c97dfb0b5d9c8a6e41ad6b6693c9d4dc6727abf875bb1d7b89d15fef402abb4598ca1421";

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
    fn from_hex_test() {
        let context = get_context(false);
        testing_env!(context);
        let contract = ProofContract::from_hex_string(&String::from(TEST_HEX));
        assert_eq!(
            contract.proof_file.check,
            CHECK
        );
    }

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
