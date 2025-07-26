use candid::Principal;
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use super::types::*;

// Define the type of memory
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Thread-local storage for the whitelist
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    // Store whitelist entries with Principal as key
    static WHITELIST: RefCell<StableBTreeMap<Principal, WhitelistEntry, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );
}

/// Add a principal to the whitelist
pub fn add_to_whitelist(args: AddToWhitelistArgs) -> WhitelistOperationResult {
    let caller = ic_cdk::caller();
    let now = time();
    
    let entry = WhitelistEntry {
        principal: args.principal,
        added_at: now,
        added_by: caller,
        expires_at: args.expires_at,
        notes: args.notes,
    };
    
    WHITELIST.with(|whitelist| {
        whitelist.borrow_mut().insert(args.principal, entry);
    });
    
    WhitelistOperationResult {
        success: true,
        message: Some(format!("Principal {} added to whitelist", args.principal)),
    }
}

/// Remove a principal from the whitelist
pub fn remove_from_whitelist(args: RemoveFromWhitelistArgs) -> WhitelistOperationResult {
    WHITELIST.with(|whitelist| {
        let mut whitelist_mut = whitelist.borrow_mut();
        
        if whitelist_mut.get(&args.principal).is_some() {
            whitelist_mut.remove(&args.principal);
            WhitelistOperationResult {
                success: true,
                message: Some(format!("Principal {} removed from whitelist", args.principal)),
            }
        } else {
            WhitelistOperationResult {
                success: false,
                message: Some(format!("Principal {} not found in whitelist", args.principal)),
            }
        }
    })
}

/// Check if a principal is on the whitelist
pub fn is_whitelisted(principal: &Principal) -> bool {
    let now = time();
    
    WHITELIST.with(|whitelist| {
        let whitelist_ref = whitelist.borrow();
        
        if let Some(entry) = whitelist_ref.get(principal) {
            // Check if the entry has expired
            if let Some(expires_at) = entry.expires_at {
                if expires_at <= now {
                    return false; // Entry has expired
                }
            }
            
            true // Entry is valid
        } else {
            false // Principal not found in whitelist
        }
    })
}

/// Get all whitelist entries
pub fn get_whitelist() -> Vec<WhitelistEntry> {
    WHITELIST.with(|whitelist| {
        let whitelist_ref = whitelist.borrow();
        whitelist_ref.iter().map(|(_, entry)| entry.clone()).collect()
    })
}

/// Check if whitelist is empty
pub fn is_whitelist_empty() -> bool {
    WHITELIST.with(|whitelist| {
        let whitelist_ref = whitelist.borrow();
        whitelist_ref.is_empty()
    })
}
