#![allow(non_snake_case)]
#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, Bytes, symbol_short};

// Defining the ProofStatus structure to track the proof of existence status for documents
#[contracttype]
#[derive(Clone)]
pub struct ProofStatus {
    pub proof_count: u64,   // The total number of proofs submitted
    pub verified: u64,      // The number of verified proofs
    pub pending: u64,       // The number of pending proofs for verification
}

// Symbol constant for accessing all proofs
const ALL_PROOFS: Symbol = symbol_short!("ALL_PRFS");

// Enum for Proofbook, mapping unique proof IDs
#[contracttype]
pub enum Proofbook {
    Proof(u64),
}

// Struct for storing proof details
#[contracttype]
#[derive(Clone)]
pub struct Proof {
    pub proof_id: u64,      // Unique ID of the proof
    pub document_hash: Bytes,  // Hash of the document being proven
    pub timestamp: u64,     // Time of proof creation
    pub verified: bool,     // Whether the proof has been verified
}

#[contract]
pub struct ProofOfExistenceContract;

#[contractimpl]
impl ProofOfExistenceContract {
    // This function creates a new proof of existence for a document
    pub fn create_proof(env: Env, document_hash: Bytes) -> u64 {
        // Fetching the current proof count
        let mut proof_count: u64 = env.storage().instance().get(&ALL_PROOFS).unwrap_or(0);
        proof_count += 1;

        // Getting the current timestamp
        let time = env.ledger().timestamp();

        // Creating a new proof record
        let new_proof = Proof {
            proof_id: proof_count,
            document_hash: document_hash.clone(),
            timestamp: time,
            verified: false,
        };

        // Storing the new proof in storage
        env.storage().instance().set(&Proofbook::Proof(proof_count.clone()), &new_proof);

        // Logging proof creation event
        log!(&env, "Proof created with ID: {}", proof_count);

        // Updating the proof count in global storage
        env.storage().instance().set(&ALL_PROOFS, &proof_count);

        // Returning the newly created proof ID
        proof_count
    }

    // Function for verifying the proof of a document
    pub fn verify_proof(env: Env, proof_id: u64) {
        // Fetching the proof from storage
        let mut proof_record = Self::view_proof(env.clone(), proof_id.clone());

        // Checking if the proof is already verified
        if proof_record.verified {
            log!(&env, "Proof with ID: {} is already verified", proof_id);
            panic!("Proof already verified");
        }

        // Setting the proof to verified
        proof_record.verified = true;

        // Storing the updated proof in storage
        env.storage().instance().set(&Proofbook::Proof(proof_id.clone()), &proof_record);

        // Logging the verification event
        log!(&env, "Proof with ID: {} has been verified", proof_id);
    }

    // Function for viewing a specific proof using the proof ID
    pub fn view_proof(env: Env, proof_id: u64) -> Proof {
        let key = Proofbook::Proof(proof_id.clone());
        env.storage().instance().get(&key).unwrap_or(Proof {
            proof_id: 0,
            document_hash: Bytes::from_array(&env, &[0u8; 32]),
            timestamp: 0,
            verified: false,
        })
    }

    // Function for retrieving the status of all proofs
    pub fn view_all_proof_status(env: Env) -> ProofStatus {
        let proof_count = env.storage().instance().get(&ALL_PROOFS).unwrap_or(0);
        let verified = env.storage().instance().get(&ALL_PROOFS).unwrap_or(0); // Placeholder for actual verification count
        let pending = proof_count - verified; // Placeholder logic for pending proofs

        ProofStatus {
            proof_count,
            verified,
            pending,
        }
    }
}
