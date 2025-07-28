use candid::Principal;
use ic_cdk_macros::*;

use super::types::*;
use super::storage::*;

// Whitelist management API endpoints

#[query]
pub fn get_whitelist() -> Vec<WhitelistEntry> {
    super::storage::get_whitelist()
}

#[update]
pub fn add_to_whitelist(args: AddToWhitelistArgs) -> WhitelistOperationResult {
    super::storage::add_to_whitelist(args)
}

#[update]
pub fn remove_from_whitelist(args: RemoveFromWhitelistArgs) -> WhitelistOperationResult {
    super::storage::remove_from_whitelist(args)
}

#[query]
pub fn is_whitelisted(principal: Principal) -> bool {
    super::storage::is_whitelisted(&principal)
}
