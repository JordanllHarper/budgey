/// Creates a json file with the given path and name.
/// budgey_path -> path to the budgey directory
/// name -> name of the json file
pub fn create_json_path(root: &str, name: &str) -> String {
    format!("{}/{}.json", root, name)
}

/// Appends the given name with a '.json' extension.
pub fn create_json_file_name(name: &str) -> String {
    format!("{}.json", name)
}

/// Concatenates the given path and name with a '/' in between.
pub fn concat_paths(root: &str, new: &str) -> String {
    format!("{}/{}", root, new)
}

pub fn get_current_timestamp() -> anyhow::Result<String> {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)?
        .as_secs()
        .to_string();
    Ok(current_time)
}

pub fn round_to_two_decimals(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}

// write a test for the above
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_json_path() {
        let budgey_path = "/tmp";
        let name = "test";
        let expected = "/tmp/test.json";
        let result = create_json_path(budgey_path, name);
        assert_eq!(expected, result);
    }
}
