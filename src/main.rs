mod logger;
mod wm_logger;
mod wm;

use std::path::{PathBuf, Path};

use logger::{Logger, LogVariant, Log};
use wm_logger::WMLogger;
use wm::WM;

fn main() -> Result<(), ()> {
    let log_path = get_log_path().unwrap();

    let wm_logger = match WMLogger::new(&log_path) {
        Ok(l) => l,
        Err(msg) => {
            panic!("Failed to create logger: {msg}!");
        }
    };

    let mut fpwm = WM::new(Box::new(wm_logger));
    fpwm.init()?;
    fpwm.run();

    Ok(())
}

fn get_log_path() -> Result<PathBuf, String> {
    let mut log_path = PathBuf::from(std::env::var("HOME").unwrap());
    log_path.extend(Path::new(".fpwm/"));

    if let Err(_) = ensure_folder_exists(&log_path) {
        return Err("Failed to create logger: Could not create $HOME/.fpwm/!".into());
    }

    log_path.extend(Path::new("fpwm.log"));

    Ok(log_path)
}

fn ensure_folder_exists(path: &Path) -> Result<(), ()> {
    if let Ok(m) = std::fs::metadata(path) {
        if !m.is_dir() {
            Err(())
        }
        else {
            Ok(())
        }
    }
    else {
        match std::fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}