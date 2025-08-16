use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

mod types;
mod utils;
mod modules;

use types::*;
use utils::*;   
use modules::whitelist::*;

pub use modules::whitelist::{get_whitelist, add_to_whitelist, remove_from_whitelist, is_whitelisted};


// Define the type of memory
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Thread-local storage for the memory manager and stable storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    // Store rule modules with their IDs
    static RULE_MODULES: RefCell<StableBTreeMap<ModuleId, RuleModule, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    // Store the next module ID
    static NEXT_MODULE_ID: RefCell<ModuleId> = RefCell::new(1);
}

// Helper function to get the next module ID
fn get_next_module_id() -> ModuleId {
    NEXT_MODULE_ID.with(|id| {
        let current_id = *id.borrow();
        *id.borrow_mut() = current_id + 1;
        current_id
    })
}

#[query]
fn get_module(id: ModuleId) -> Option<RuleModule> {
    RULE_MODULES.with(|modules| modules.borrow().get(&id))
}

#[query]
fn list_modules() -> Vec<RuleModule> {
    RULE_MODULES.with(|modules| {
        let modules_ref = modules.borrow();
        modules_ref.iter().map(|(_, module)| module.clone()).collect()
    })
}

#[update]
fn upload_module(args: UploadModuleArgs) -> ModuleId {
    let caller = ic_cdk::caller();
    let now = time();
    
    let module_id = get_next_module_id();
    
    let module = RuleModule {
        id: module_id,
        status: ModuleStatus::Active,
        metadata: ModuleMetadata {
            name: args.name,
            description: args.description,
            version: args.version,
            created_at: now,
            updated_at: now,
            created_by: caller,
        },
        wasm_module: args.wasm_module,
    };
    
    RULE_MODULES.with(|modules| {
        modules.borrow_mut().insert(module_id, module);
    });
    
    module_id
}

#[update]
fn update_module(args: UpdateModuleArgs) -> Result<(), String> {
    let module_id = args.id;
    
    RULE_MODULES.with(|modules| {
        let mut modules_mut = modules.borrow_mut();
        
        if let Some(mut module) = modules_mut.get(&module_id) {
            // Update fields if provided
            if let Some(name) = args.name {
                module.metadata.name = name;
            }
            
            if let Some(description) = args.description {
                module.metadata.description = description;
            }
            
            if let Some(version) = args.version {
                module.metadata.version = version;
            }
            
            if let Some(wasm_module) = args.wasm_module {
                module.wasm_module = wasm_module;
            }
            
            if let Some(status) = args.status {
                module.status = status;
            }
            
            // Update the updated_at timestamp
            module.metadata.updated_at = time();
            
            // Insert the updated module
            modules_mut.insert(module_id, module);
            Ok(())
        } else {
            Err(format!("Module with ID {} not found", module_id))
        }
    })
}

#[update]
fn remove_module(id: ModuleId) -> Result<(), String> {
    RULE_MODULES.with(|modules| {
        let mut modules_mut = modules.borrow_mut();
        
        if modules_mut.get(&id).is_some() {
            modules_mut.remove(&id);
            Ok(())
        } else {
            Err(format!("Module with ID {} not found", id))
        }
    })
}


mod engine;

#[query]
fn is_allowed(args: IsAllowedArgs) -> IsAllowedResult {
    // Use the rules engine to apply all rules
    engine::apply_all_rules(&args)
}

// Additional API endpoints for rule configuration

#[query]
fn is_rule_enabled(rule_name: String) -> bool {
    engine::is_rule_enabled(&rule_name)
}

#[update]
fn set_rule_enabled(rule_name: String, enabled: bool) -> Result<(), String> {
    engine::set_rule_enabled(&rule_name, enabled)
}