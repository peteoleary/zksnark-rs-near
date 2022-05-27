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

use super::proof_file::{ProofFile};

fn do_string_output(output_path: Option<std::path::PathBuf>, output_string: String) {

    let mut out_writer = match output_path {
        Some(x) => {
            Box::new(File::create(&x).unwrap()) as Box<dyn Write>
        }
        None => Box::new(stdout()) as Box<dyn Write>,
    };
    
    out_writer.write_all(output_string.as_bytes());
}

pub fn do_binary_output(output_path: std::path::PathBuf,  buf: Vec<u8>) -> File {
    let mut file = File::create(&output_path).unwrap();
    file.write_all(&buf);
    return file
}

pub fn read_bin_file<V: BorshDeserialize>(setup_path: std::path::PathBuf) -> V {
    let setup_bin = &*::std::fs::read(setup_path).unwrap();
    return V::try_from_slice(setup_bin).unwrap();
}

// arbitrary check value addeed to the *File structs so that we can ensure they are deserialized correctly
// in unit tests
pub const CHECK: u32 = 0xABAD1DEA;

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone)]
pub struct SetupFile {
    pub check: u32,
    pub code: String,
    pub qap: QAP<CoefficientPoly<FrLocal>>,
    pub sigmag1: SigmaG1<G1Local>,
    pub sigmag2: SigmaG2<G2Local>
}

impl SetupFile {

    pub fn from_zk(code: &str) -> SetupFile {
        let qap: QAP<CoefficientPoly<FrLocal>> = ASTParser::try_parse(code).unwrap().into();
    
        let (sigmag1, sigmag2) = groth16::setup(&qap);
    
        return SetupFile {check: CHECK, qap: qap, code: String::from(code), sigmag1: sigmag1, sigmag2: sigmag2};
    }

    pub fn from_zk_file(zk_path: std::path::PathBuf) -> SetupFile {

        let code = &*::std::fs::read_to_string(zk_path).unwrap();
        return SetupFile::from_zk(code);
    }

    pub fn to_file(&self, output_path: std::path::PathBuf) {
        let encoded =  self.try_to_vec().unwrap();
        do_binary_output(output_path, encoded);
    }

    pub fn verify(&self, assignments: &[FrLocal], proof: ProofFile) -> bool {
        let sigmas = (self.sigmag1.clone(), self.sigmag2.clone());
        return groth16::verify::<CoefficientPoly<FrLocal>, _, _, _, _> (
            sigmas,
            assignments,
            proof.proof
        );
    }

    pub fn verify_from_file(&self, assignments: &[FrLocal], proof_path: std::path::PathBuf) -> bool {
        let proof: ProofFile = read_bin_file(proof_path);
        return self.verify(assignments, proof)
    }

    pub fn from_file(setup_path: std::path::PathBuf) -> SetupFile {
        return read_bin_file(setup_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
    fn try_setup_test() {
        SetupFile::from_zk_file(PathBuf::from("test_programs/simple.zk")).to_file(PathBuf::from("output/simple.setup.bin"));
        assert!(true);
    }

    #[test]
    fn try_read_setup_test() {
        let setup: SetupFile = SetupFile::from_file(PathBuf::from("output/simple.setup.bin"));
        assert!(setup.check == CHECK)
    }

    #[test]
    fn try_proof_test() {
        ProofFile::from_setup_file(&input_assignments(), PathBuf::from("output/simple.setup.bin")).to_file(PathBuf::from("output/simple.proof.bin"));
        assert!(true);
    }

    #[test]
    fn try_verify_test() {
        assert!(SetupFile::from_file(PathBuf::from("output/simple.setup.bin")).verify_from_file(&output_assignments(), PathBuf::from("output/simple.proof.bin")));
    }

    #[test]
    fn try_read_proof_test() {
        let setup: ProofFile = ProofFile::from_file(PathBuf::from("output/simple.proof.bin"));
        assert!(setup.check == CHECK)
    }
}