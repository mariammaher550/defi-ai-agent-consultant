mod storage;
mod types;

pub use storage::*;
pub use types::*;

use crate::rules_engine::types::{IsAllowedArgs, IsAllowedResult};
use candid::Principal;

/// Check if a receiver is on the whitelist
pub fn is_receiver_allowed(receiver: &Principal) -> bool {
    is_whitelisted(receiver)
}

/// Apply whitelist rule to check if a transfer is allowed
pub fn apply_whitelist_rule(args: &IsAllowedArgs) -> IsAllowedResult {
    if is_receiver_allowed(&args.to) {
        IsAllowedResult {
            allowed: true,
            message: None,
        }
    } else {
        IsAllowedResult {
            allowed: false,
            message: Some(format!("Receiver {} is not on the whitelist", args.to)),
        }
    }
}
