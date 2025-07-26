pub mod whitelist;

// Module registration - add new modules here
pub fn get_all_modules() -> Vec<&'static str> {
    vec!["whitelist"]
}
