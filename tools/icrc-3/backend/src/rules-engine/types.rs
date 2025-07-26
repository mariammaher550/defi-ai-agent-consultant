use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use std::borrow::Cow;
use serde::Serialize;

/// Unique identifier for a rule module
pub type ModuleId = u64;

/// Status of a rule module
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ModuleStatus {
    Active,
    Inactive,
}

/// Metadata about a rule module
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ModuleMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: Principal,
}

/// A rule module that can be used to validate transfers
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RuleModule {
    pub id: ModuleId,
    pub status: ModuleStatus,
    pub metadata: ModuleMetadata,
    pub wasm_module: Vec<u8>,  // The actual WASM binary code
}

/// Arguments for the isAllowed function
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct IsAllowedArgs {
    pub from: Principal,
    pub to: Principal,
    pub amount: u64,
    pub fee: u64,
    pub memo: Option<Vec<u8>>,
}

/// Result of the isAllowed function
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct IsAllowedResult {
    pub allowed: bool,
    pub message: Option<String>,
}

/// Arguments for uploading a module
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UploadModuleArgs {
    pub name: String,
    pub description: String,
    pub version: String,
    pub wasm_module: Vec<u8>,
}

/// Arguments for updating a module
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateModuleArgs {
    pub id: ModuleId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub wasm_module: Option<Vec<u8>>,
    pub status: Option<ModuleStatus>,
}

/// Implement Storable for RuleModule for stable storage
impl Storable for RuleModule {
    fn to_bytes(&self) -> Cow<[u8]> {
        // For now, we'll use a simple serialization approach
        // In a production system, you might want to use a more efficient binary format
        let bytes = candid::encode_one(self).unwrap();
        Cow::Owned(bytes)
    }
    
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        // Deserialize from bytes
        match candid::decode_one(&bytes) {
            Ok(module) => module,
            Err(_) => {
                // Return a default module in case of deserialization error
                // In production, you'd want better error handling
                RuleModule {
                    id: 0,
                    status: ModuleStatus::Inactive,
                    metadata: ModuleMetadata {
                        name: String::new(),
                        description: String::new(),
                        version: String::new(),
                        created_at: 0,
                        updated_at: 0,
                        created_by: Principal::anonymous(),
                    },
                    wasm_module: Vec::new(),
                }
            }
        }
    }
}

impl BoundedStorable for RuleModule {
    // Set a reasonable maximum size for a rule module
    // This would depend on your specific requirements
    const MAX_SIZE: u32 = 10 * 1024 * 1024; // 10MB max size
    const IS_FIXED_SIZE: bool = false;
}