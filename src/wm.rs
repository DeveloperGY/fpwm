use x11::xlib::*;

/// The core of fpwm. It represents an X11 Window manger.
pub struct WM {
    display: *mut Display
}

/// 
impl WM {
    pub fn new(display: *mut Display) -> Self {
        Self {
            display
        }
    }

    pub fn create() -> Self {
        let display = match WM::connect() {
            Ok(d) => d,
            Err(e) => panic!("{}", e)
        };

        WM::new(display)
    }
}

/// Helpers for creating the window manager
impl WM {
    fn connect() -> Result<*mut Display, &'static str> {
        unsafe {
            let display = XOpenDisplay(std::ptr::null());

            if display.is_null() {
                Err("Failed to connect to X server")
            }
            else {
                Ok(display)
            }
        }
    }
}