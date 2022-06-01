#  zkSNARKs on the NEAR Blockchain

> Nothing in this project is ready for production, this is meant for educational (mine mostly) and illustrative purposes only.

This system is comprised of several pieces:

* The central components of the system are based on https://github.com/republicprotocol/zksnark-rs to which I have added code to serialize the zkSNARKs using both Borsh and Serde. Take a look at the SetupFile and ProofFile structs. These two objects represent the 2 artifacts that are used by the Verifier and Prover respectively in an exchange of data to support a business application such as a loan application without disclosing more private information than necessary. I've written more about this here https://github.com/peteoleary/loan-zkp-blockchain and done some work using the Circom ZKP system. When I get a chance I will write a more detailed business process discussion for this repo as well.
* The zkSNARK SetupFile and ProofFile structs are intended to be used in 2 ways: in a command line tool (in the `cli` folder) and also in a NEAR smart contract (`contracts` folder). The general idea is that in a business transaction the Prover and Verifier would each create their respective artifacts off-chain to preserve privacy and secrecy then commit the results to the blockchain along with contract code to perform verification on-chain.
* A very important thing to note about this system is that the beneficiary (see the writeup in loan-zkp-blockchain) of this business transaction would be able to see that the Prover and Verifier have completed the transaction and have done so without disclosing private information. This represents a fundamentally different way that business transactions can and will be done in the Web3 world.
* This project depends on a forked version of rabe-bn https://github.com/peteoleary/rabe-bn which has been modified to support both Borsh and Serde serialization. The original version supported both but not at the same time. For NEAR smart contracts, Borsh is used for on-chain storage and Serde for parameter passing.
* This project also depends on a forked version of getrandom https://github.com/peteoleary/getrandom/tree/2022-may-31-add-noop-feature where I have implemented a "noop" (short for no operation). Please read read the README in that project for details of why I decided to do this.

## Why?

This project exists because I wanted to explore the NEAR blockchain in more detail. In particular I wanted to to discover how much existing Rust code can be used on the blockchain. Towards this end I picked a project which I thought would require serialization and deserialization of fairly complicated structs and have both on-chain and off-chain components. Also, I believe there are not enough examples of using zkSNARKs for privacy-protected business transactions and my intention is to change that.

## What next?

The circuit language used in zksnark-rs is rather simple compared to Circom (https://github.com/iden3/circom) and the latter also has a library of re-usable circuits which make it much easier to write business logic. It's also written in Rust. As a next step, I may try to bring Circom circuits onto NEAR and perhaps create a cross-chain zkSNARK protocol for privacy protected business transactions.

## TODO

* Get contracts working and calling each other
* ~~Make sure CLI still working~~
* Extend CLI to create and verify contracts
* Add "why" to this README doc
* Add references to other projects in this doc
* Copy in NEAR contract installation instructions
* Add notes to zksnark project: switch to rabe-bin, serialization changes
* Rename all packages in Cargo.toml file
* ~~Fix tests in zksnark project~~
* ~~Add contract tests~~
* Add README to contracts project
* Change all Cargo.toml references from local to github.com