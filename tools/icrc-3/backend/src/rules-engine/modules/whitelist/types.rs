use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

/// Whitelist entry with optional expiration
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WhitelistEntry {
    pub principal: Principal,
    pub added_at: u64,
    pub added_by: Principal,
    pub expires_at: Option<u64>,
    pub notes: Option<String>,
}

/// Arguments for adding a principal to the whitelist
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AddToWhitelistArgs {
    pub principal: Principal,
    pub expires_at: Option<u64>,
    pub notes: Option<String>,
}

/// Arguments for removing a principal from the whitelist
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RemoveFromWhitelistArgs {
    pub principal: Principal,
}

/// Result of whitelist operations
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WhitelistOperationResult {
    pub success: bool,
    pub message: Option<String>,
}
