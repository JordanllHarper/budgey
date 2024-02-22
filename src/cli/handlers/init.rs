pub trait InitHandler {
    fn handle(&self, repo_name: &str) -> anyhow::Result<()>;
}
pub struct InitHandlerImpl {}

impl InitHandlerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl InitHandler for InitHandlerImpl {
    fn handle(&self, repo_name: &str) -> anyhow::Result<()> {
        // TODO: Create the repo and the main pile
        todo!()
    }
}
