extern crate near_sdk;
use self::near_sdk::{env, log, metadata, near_bindgen, AccountId};

use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

use zksnark::setup_file::{SetupFile, CHECK, do_binary_output, read_bin_file};

#[derive(BorshDeserialize, BorshSerialize, Debug)]

struct SetupContract {
    proof_file: ProofFile;
}

impl SetupContract {
    
}