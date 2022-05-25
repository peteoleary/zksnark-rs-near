extern crate near_sdk;
use self::near_sdk::{env, log, metadata, near_bindgen, AccountId};

use borsh::{BorshSerialize, BorshDeserialize};

use zksnark::proof_file::{ProofFile};

#[derive(BorshDeserialize, BorshSerialize, Debug)]

struct ProofContract {
    proof_file: ProofFile
}

impl ProofContract {
    
}