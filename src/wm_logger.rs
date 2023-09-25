use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

use super::{Logger, LogVariant, Log};

pub struct WMLogger {
    log_file: File
}

// Construction of a WMLogger
impl WMLogger {
    pub fn new(log_path: &PathBuf) -> Result<Self, String> {
        let log_file = WMLogger::open_log_file(log_path)?;
        
        Ok(Self {
            log_file
        })
    }

    fn open_log_file(log_path: &PathBuf) -> Result<File, String> {
        if let Ok(f) = File::create(log_path) {
            Ok(f)
        }
        else {
            Err("failed to open log file".into())
        }
    }
}

// Logging Handlers
impl WMLogger {
    fn log_notif(&mut self, msg: &str) {
        let log_msg = format!("[INFO]: {}\n", msg);
        let _ = self.log_file.write_all(log_msg.as_bytes());
    }

    fn log_warning(&mut self, msg: &str) {
        let log_msg = format!("[WARNING]: {}\n", msg);
        let _ = self.log_file.write_all(log_msg.as_bytes());
    }

    fn log_error(&mut self, msg: &str) {
        let log_msg = format!("[ERROR]: {}\n", msg);
        let _ = self.log_file.write_all(log_msg.as_bytes());
    }
}

// Implementation of the Logger interface
impl Logger for WMLogger {
    fn log(&mut self, log: &Log) {
        match log.variant {
            LogVariant::Notif => self.log_notif(&log.message),
            LogVariant::Warn  => self.log_warning(&log.message),
            LogVariant::Error => self.log_error(&log.message)
        };
    }
}