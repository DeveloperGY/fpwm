mod logger;
mod wm_logger;
mod wm;

use std::path::{PathBuf, Path};

use logger::{Logger, LogVariant, Log};
use wm_logger::WMLogger;
use wm::WM;

fn main() {
    let mut log_path = PathBuf::from(std::env::var("HOME").unwrap());
    log_path.extend(Path::new("/.fpwm/fpwm.log"));

    let wm_logger = match WMLogger::new(&log_path) {
        Ok(l) => l,
        Err(msg) => {
            panic!("Failed to create logger: {msg}");
        }
    };

    let mut fpwm = WM::new(Box::new(wm_logger));
    fpwm.test();
}