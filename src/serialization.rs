use std::str::FromStr;

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

fn do_string_output(output_path: Option<std::path::PathBuf>, output_string: String) {

    let mut out_writer = match output_path {
        Some(x) => {
            Box::new(File::create(&x).unwrap()) as Box<dyn Write>
        }
        None => Box::new(stdout()) as Box<dyn Write>,
    };
    
    out_writer.write_all(output_string.as_bytes());
}

fn do_binary_output(output_path: std::path::PathBuf,  buf: Vec<u8>) -> File {
    let mut file = File::create(&output_path).unwrap();
    file.write_all(&buf);
    return file
}

fn read_bin_file<V: BorshDeserialize>(setup_path: std::path::PathBuf) -> V {
    let setup_bin = &*::std::fs::read(setup_path).unwrap();
    return V::try_from_slice(setup_bin).unwrap();
}

// arbitrary check value addeed to the *File structs so that we can ensure they are deserialized correctly
// in unit tests
const CHECK: u32 = 0xABAD1DEA;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct SetupFile {
    check: u32,
    code: String,
    qap: QAP<CoefficientPoly<FrLocal>>,
    sigmag1: SigmaG1<G1Local>,
    sigmag2: SigmaG2<G2Local>
}

impl SetupFile {

    fn setup() {
        
    }

    fn from_file(zk_path: std::path::PathBuf, output_path: std::path::PathBuf) {

        let code = &*::std::fs::read_to_string(zk_path).unwrap();
        let qap: QAP<CoefficientPoly<FrLocal>> = ASTParser::try_parse(code).unwrap().into();
    
        let (sigmag1, sigmag2) = groth16::setup(&qap);
    
        let setup_file_object = SetupFile {check: CHECK, qap: qap, code: String::from(code), sigmag1: sigmag1, sigmag2: sigmag2};
    
        // do_string_output(output_path, json::encode(&setup_file_object).unwrap());
        let encoded =  setup_file_object.try_to_vec().unwrap();
        do_binary_output(output_path, encoded);
    }

    fn to_file() {

    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct ProofFile {
    check: u32,
    proof: Proof<G1Local, G2Local>
}

impl ProofFile {
    fn proof(assignments: &[FrLocal], setup_path: std::path::PathBuf, output_path: std::path::PathBuf) 
    // where F: Clone + zksnark::field::Field + FromStr + PartialEq, 
    {

    let setup: SetupFile = read_bin_file(setup_path);
    let weights = groth16::weights(&setup.code, assignments).unwrap();

    let proof = groth16::prove(&setup.qap, (&setup.sigmag1, &setup.sigmag2), &weights);
    let proof_file = ProofFile {check: CHECK, proof: proof};
    let encoded =  proof_file.try_to_vec().unwrap();
    do_binary_output(output_path, encoded);
}
}

fn verify(assignments: &[FrLocal], setup_path: std::path::PathBuf, proof_path: std::path::PathBuf) -> bool {
    let setup: SetupFile = read_bin_file(setup_path);
    let proof: ProofFile = read_bin_file(proof_path);
    return groth16::verify::<CoefficientPoly<FrLocal>, _, _, _, _> (
        (setup.sigmag1, setup.sigmag2),
        assignments,
        proof.proof
    );
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
        setup(PathBuf::from("../test_programs/simple.zk"), PathBuf::from("simple.setup.bin"));
        assert!(true);
    }

    #[test]
    fn try_read_setup_test() {
        let setup: SetupFile = read_bin_file(PathBuf::from("simple.setup.bin"));
        assert!(setup.check == CHECK)
    }

    #[test]
    fn try_proof_test() {
        proof(&input_assignments(), PathBuf::from("simple.setup.bin"), PathBuf::from("simple.proof.bin"));
        assert!(true);
    }

    #[test]
    fn try_verify_test() {
        assert!(verify(&output_assignments(), PathBuf::from("simple.setup.bin"), PathBuf::from("simple.proof.bin")));
    }

    #[test]
    fn try_read_proof_test() {
        let setup: ProofFile = read_bin_file(PathBuf::from("simple.proof.bin"));
        assert!(setup.check == CHECK)
    }
}
