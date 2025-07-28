use candid::Principal;
use ic_cdk::api::time;

use crate::rules_engine::types::{IsAllowedArgs, IsAllowedResult, ModuleStatus, RuleModule};
use crate::rules_engine::RULE_MODULES;
use crate::rules_engine::modules::whitelist;

/// Apply a single rule to check if a transfer is allowed
pub fn apply_rule(rule_name: &str, args: &IsAllowedArgs) -> IsAllowedResult {
    match rule_name {
        "whitelist" => whitelist::apply_whitelist_rule(args),
        // Add other rules here as needed
        _ => IsAllowedResult {
            allowed: true, // Default to allowing if rule not found
            message: Some(format!("Rule '{}' not found, defaulting to allowed", rule_name)),
        },
    }
}

/// Apply all active rules to check if a transfer is allowed
pub fn apply_all_rules(args: &IsAllowedArgs) -> IsAllowedResult {
    // Get all active modules
    let active_modules = RULE_MODULES.with(|modules| {
        let modules_ref = modules.borrow();
        modules_ref
            .iter()
            .filter(|(_, module)| module.status == ModuleStatus::Active)
            .map(|(_, module)| module.clone())
            .collect::<Vec<RuleModule>>()
    });
    
    // In a real implementation, you would invoke each module's WASM code
    // For now, we'll just check if any rule denies the transfer
    
    // Apply built-in rules first
    let whitelist_result = apply_rule("whitelist", args);
    if !whitelist_result.allowed {
        return whitelist_result;
    }
    
    // Future: Apply WASM module rules
    // for module in active_modules {
    //     let result = apply_wasm_module(&module, args);
    //     if !result.allowed {
    //         return result;
    //     }
    // }
    
    // If all rules pass, allow the transfer
    IsAllowedResult {
        allowed: true,
        message: None,
    }
}

/// Check if a specific rule is enabled
pub fn is_rule_enabled(rule_name: &str) -> bool {
    match rule_name {
        "whitelist" => true, // Whitelist rule is always enabled for now
        // Add other built-in rules here
        _ => false,
    }
}

/// Configure whether a specific rule is enabled
pub fn set_rule_enabled(rule_name: &str, enabled: bool) -> Result<(), String> {
    // This would be implemented to enable/disable specific rules
    // For now, we'll just return an error since it's not implemented
    Err(format!("Setting rule '{}' enabled={} is not implemented", rule_name, enabled))
}
