use log::{error, info};

pub fn init_logging() {
    env_logger::init();
    info!("Logging initialized");
}
