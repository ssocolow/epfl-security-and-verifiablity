use borsh::{io::Error, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use tfhe::{ConfigBuilder, generate_keys, set_server_key, ServerKey, FheUint32};
use tfhe::prelude::*;
use rand::prelude::*;

use sdk::{Digestable, HyleContract, RunResult};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum PsiAction {
    // Bob adds his set elements
    AddServerElement { element: u32 },
    
    // Alice sends her encrypted element
    CheckIntersection {
        encrypted_element: FheUint32,
        server_key: ServerKey,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct PsiContract {
    // Bob's set elements
    pub server_set: Vec<u32>,
}

impl HyleContract for PsiContract {
    fn execute(&mut self, contract_input: &sdk::ContractInput) -> RunResult {
        let (action, ctx) = sdk::utils::parse_raw_contract_input::<PsiAction>(contract_input)?;

        match action {
            PsiAction::AddServerElement { element } => {
                self.server_set.push(element);
                Ok((format!("Added element to server set"), ctx, vec![]))
            },
            
            PsiAction::CheckIntersection { encrypted_element, server_key } => {
                // Set up FHE context
                set_server_key(server_key);
                
                // Generate random mask
                let mut rng = rand::thread_rng();
                let mask: u32 = rng.gen();
                let e_mask = FheUint32::encrypt_trivial(mask);
                
                // For each server element, check equality and mask result
                let e_server_elements = self.server_set
                    .iter()
                    .map(|&x| FheUint32::encrypt_trivial(x));

                // Check if encrypted_element equals any server element
                let result = e_server_elements
                    .map(|e_server_elem| {
                        // Compute (encrypted_element - server_elem) * mask
                        let diff = &encrypted_element - &e_server_elem;
                        &diff * &e_mask
                    })
                    .fold(FheUint32::encrypt_trivial(0u32), |acc, x| &acc + &x);

                Ok((
                    "Intersection check complete".to_string(),
                    ctx,
                    vec![result.into()]
                ))
            }
        }
    }
}

// Implementation of required traits
impl PsiContract {
    pub fn new() -> Self {
        Self { server_set: Vec::new() }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        borsh::to_vec(self)
    }
}

impl Digestable for PsiContract {
    fn as_digest(&self) -> sdk::StateDigest {
        sdk::StateDigest(borsh::to_vec(self).expect("Failed to encode PSI contract"))
    }
}

impl From<sdk::StateDigest> for PsiContract {
    fn from(state: sdk::StateDigest) -> Self {
        borsh::from_slice(&state.0)
            .map_err(|_| "Could not decode PSI contract state".to_string())
            .unwrap()
    }
}
