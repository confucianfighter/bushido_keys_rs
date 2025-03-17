use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Once;

static INIT_LOGGER: Once = Once::new();

/// Initialize the logger for tests. This function is safe to call multiple times
/// as it will only initialize the logger once.
static mut INITIALIZED: bool = false;
pub fn init_test_logger() {
    if unsafe { INITIALIZED } {
        return;
    }
    unsafe { INITIALIZED = true };
    INIT_LOGGER.call_once(|| {
        let home_dir = env::var("USERPROFILE").expect("Failed to get home directory");
        let log_dir = PathBuf::from(home_dir).join(".bushido_keys_config");
        std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");
        let log_path = log_dir.join("bushido_test.log");

        let file = File::create(&log_path).expect("Failed to create log file");

        let mut builder = Builder::new();
        builder
            .write_style(WriteStyle::Always)
            .filter(None, LevelFilter::Debug)
            // Write to both stderr and file
            .target(env_logger::Target::Pipe(Box::new(file)))
            .init();

        println!("Test logs will be written to: {:?}", log_path);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_init() {
        init_test_logger();
        log::info!("Test logger initialized successfully");
        log::debug!("Debug logging is enabled");
    }
}
