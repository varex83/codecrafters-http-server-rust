use std::sync::OnceLock;

pub static FILES_ROOT_DIR: OnceLock<String> = OnceLock::new();
