#[derive(Clone, Copy)]
pub enum LogVariant {
    Notif,
    Error,
    Warn
}

pub trait Logger {
    fn log(&mut self, log: &Log);
}

pub struct Log {
    pub variant: LogVariant,
    pub message: String
}

impl Log {
    pub fn new(variant: LogVariant, message: &str) -> Self {
        Self {
            variant,
            message: message.to_string()
        }
    }
}