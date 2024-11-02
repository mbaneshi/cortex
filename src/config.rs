#[derive(Debug)]
pub struct Config {
    pub db_path: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            db_path: String::from("app_data.db"),
        }
    }
}
