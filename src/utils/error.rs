pub trait UseCaseError {
    fn get_user_message(&self) -> String;
}
