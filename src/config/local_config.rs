use std::env;

#[derive(Debug)]
pub struct LocalConfig {
    /// The path to the /budgey directory  
    pub budgey_dir: String,
}

impl Default for LocalConfig {
    fn default() -> Self {
        let budgey_dir = env!("HOME").to_string() + "/budgey";
        LocalConfig { budgey_dir }
    }
}
impl LocalConfig {
    pub fn test() -> Self {
        let budgey_dir = std::env::var("BUDGEY_TEST").unwrap();
        LocalConfig { budgey_dir }
    }
}
