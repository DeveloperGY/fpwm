mod x;

use x11::xlib::*;
use super::{Logger, Log, LogVariant};
use x::X;

// Whether or not the window manager is allowed to control the display
static mut CAN_ASCEND: bool = false;

pub struct WM {
    logger: Box<dyn Logger>,
    display: *mut Display,
    root: Window
}

// Window Manager Creation
impl WM {
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self {
            logger,
            display: std::ptr::null_mut(),
            root: 0
        }
    }
}

// Window Manager Initialization
impl WM {
    pub fn init(&mut self) -> Result<(), ()> {
        self.display = match X::connect() {
            Ok(d) => d,
            Err(_) => {
                let log = Log::new(
                    LogVariant::Error,
                    "Failed to initialize fpwm, failed to connect to the X server!"
                );

                self.logger.log(&log);
                return Err(());
            }
        };

        self.root = X::get_root_window(self.display);

        X::ascend(self.display, self.root);

        if unsafe{!CAN_ASCEND} {
            let log = Log::new(
                LogVariant::Error,
                "Failed to ascend fpwm, another window manager is running!"
            );

            self.logger.log(&log);
            Err(())
        }
        else {
            Ok(())
        }
    }
}

impl WM {
    pub fn run(&mut self) {
        let log = Log::new(
            LogVariant::Notif,
            "FPWM Successfully Started!"
        );
        self.logger.log(&log);
    }
}