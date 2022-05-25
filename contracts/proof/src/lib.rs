extern crate near_sdk;
use self::near_sdk::{env, log, metadata, near_bindgen, AccountId};

extern crate borsh;
use self::borsh::{BorshSerialize, BorshDeserialize};

use super::setup_file::{SetupFile, CHECK, do_binary_output, read_bin_file};

#[derive(BorshDeserialize, BorshSerialize, Debug)]

struct ProofContract {
    proof_file: ProofFile;
}

impl ProofContract {
    
}