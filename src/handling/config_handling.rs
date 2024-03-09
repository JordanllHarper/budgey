use crate::config::local_config::LocalConfig;

trait ConfigRepository {
    fn get_local_config(config_file_path: &str) -> LocalConfig;
}
