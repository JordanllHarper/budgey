pub mod io_operations {

    /// Creates the budgey root directory if it is not already created.
    /// We don't want to return an error if the file already exists (as it should just do nothing in
    /// that case).

    /// Create a path for the new pile including json file
    pub fn create_json_path(path_to_pile: &str, pile_name: &str) -> String {
        format!("{path_to_pile}/{pile_name}.json")
    }
}
#[cfg(test)]
mod io_operations_tests {
    use crate::io_operations::io_operations::io_operations;

    #[test]
    fn test_create_pile_json_path() {
        let path = "~/budgey";
        let pile_name = "test";
        let expected = "~/budgey/test.json";
        let actual = io_operations::create_json_path(path, pile_name);
        assert_eq!(expected, actual);
    }
}
