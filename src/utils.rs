/// Creates a json file with the given path and name.
/// budgey_path -> path to the budgey directory
/// name -> name of the json file
pub fn create_json_path(budgey_path: &str, name: &str) -> String {
    format!("{}/{}.json", budgey_path, name)
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
