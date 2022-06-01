
use std::str::FromStr;

use clap::{Parser, Subcommand};

extern crate zksnark;
use zksnark::groth16::fr::FrLocal;
use zksnark::setup_file::{SetupFile};
use zksnark::proof_file::{ProofFile};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The command to execute
    #[clap(subcommand)]
    command: Commands
}

fn parse_assignment_string(s: &str) -> Vec<FrLocal> {
    return s.split(',').map(|item| FrLocal::from_str(item).unwrap()).into_iter().collect::<Vec<FrLocal>>();
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    SetupFromZK {
        #[clap(long, parse(from_os_str))]
        zk_path: Option<std::path::PathBuf>,
        #[clap(long, parse(from_os_str))]
        output_path: Option<std::path::PathBuf>
    },
    SetupToHex {
        #[clap(long, parse(from_os_str))]
        setup_path: Option<std::path::PathBuf>
    },
    Proof {
        #[clap(long)]
        assignments: Option<String>,
        #[clap(long, parse(from_os_str))]
        setup_path: Option<std::path::PathBuf>,
        #[clap(long, parse(from_os_str))]
        output_path: Option<std::path::PathBuf>
    },
    Verify {
        #[clap(long)]
        assignments: Option<String>,
        #[clap(long, parse(from_os_str))]
        setup_path: Option<std::path::PathBuf>,
        #[clap(long, parse(from_os_str))]
        proof_path: Option<std::path::PathBuf>
    }
}

// command line example from https://github.com/clap-rs/clap/blob/v3.1.18/examples/git-derive.rs

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::SetupFromZK { zk_path, output_path }  => SetupFile::from_zk_file(zk_path.unwrap()).to_file(output_path.unwrap()),
        Commands::SetupToHex { setup_path }  => {
            let setup = SetupFile::from_file(setup_path.unwrap());
           
            println!("{}", setup.to_hex_string());
        },
        Commands::Proof { assignments, setup_path, output_path }  => {
            ProofFile::from_setup_file(&parse_assignment_string(&assignments.unwrap()[..]), setup_path.unwrap()).to_file(output_path.unwrap());
        },
        Commands::Verify { assignments, setup_path, proof_path }  => {
            SetupFile::from_file(setup_path.unwrap()).verify_from_file(&parse_assignment_string(&assignments.unwrap()[..]), proof_path.unwrap());
        },
        _ => println!("unknown command!"),
    }
    
}