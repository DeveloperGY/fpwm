use super::{Logger, Log, LogVariant};

pub struct WM {
    logger: Box<dyn Logger>
}

impl WM {
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self {
            logger
        }
    }

    pub fn test(&mut self) {
        let log = Log::new(LogVariant::Notif, "Get notified");

        self.logger.log(&log)
    }
}