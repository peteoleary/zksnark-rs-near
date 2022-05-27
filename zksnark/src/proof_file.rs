use std::io::{stdout, Write};
use std::fs::{File};

use std::string::String;

use groth16;
use groth16::{SigmaG1, SigmaG2};
use groth16::fr::{G1Local, G2Local, Proof, QAP, FrLocal};
use groth16::coefficient_poly::{CoefficientPoly};
use groth16::circuit::{ASTParser, TryParse};

extern crate borsh;
use self::borsh::{BorshSerialize, BorshDeserialize};

extern crate serde;
use self::serde::{Serialize, Deserialize};

use super::setup_file::{SetupFile, CHECK, do_binary_output, read_bin_file};

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone)]
pub struct ProofFile {
    pub check: u32,
    pub proof: Proof<G1Local, G2Local>
}

impl ProofFile {
    
    pub fn from_setup(assignments: &[FrLocal], setup: SetupFile) -> ProofFile {
        let weights = groth16::weights(&setup.code, assignments).unwrap();

        let proof = groth16::prove(&setup.qap, (&setup.sigmag1, &setup.sigmag2), &weights);
        return ProofFile {check: CHECK, proof: proof};
    }

    pub fn from_setup_file(assignments: &[FrLocal], setup_path: std::path::PathBuf) -> ProofFile
    // where F: Clone + zksnark::field::Field + FromStr + PartialEq, 
    {
        let setup = SetupFile::from_file(setup_path);
        return ProofFile::from_setup(assignments, setup);
    }

    pub fn from_file(proof_path: std::path::PathBuf) -> ProofFile {
        return read_bin_file(proof_path)
    }

    pub fn to_file(&self, output_path: std::path::PathBuf) {
        let encoded =  self.try_to_vec().unwrap();
        do_binary_output(output_path, encoded);
    }
}