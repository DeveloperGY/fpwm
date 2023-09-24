mod logger;
mod wm_logger;
mod wm;

use std::path::PathBuf;

use logger::{Logger, LogVariant, Log};
use wm_logger::WMLogger;
use wm::WM;

fn main() {
    let log_path = PathBuf::from("/var/log/fpwm.log");
    let wm_logger = match WMLogger::new(&log_path) {
        Ok(l) => l,
        Err(msg) => {
            panic!("Failed to create logger: {msg}");
        }
    };

    let mut fpwm = WM::new(Box::new(wm_logger));
    fpwm.test();
}