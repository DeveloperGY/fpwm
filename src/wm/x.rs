use x11::xlib::*;

use super::CAN_ASCEND;

/// Represents Xlib
pub struct X;

impl X {

    /// Connects to the X server
    pub fn connect() -> Result<*mut Display, String> {
        unsafe {
            let display = XOpenDisplay(std::ptr::null());

            if display.is_null() {
                Err("Failed to connect to display".into())
            }
            else {
                Ok(display)
            }
        }
    }

    pub fn get_root_window(display: *mut Display) -> Window {
        unsafe {
            XDefaultRootWindow(display)
        }
    }

    pub fn ascend(display: *mut Display, root: Window) {
        unsafe {
            XSetErrorHandler(Some(X::handle_ascension_error));
            XSelectInput(
                display,
                root,
                SubstructureNotifyMask | SubstructureRedirectMask
            );
            XSync(display, 1);
            XSetErrorHandler(None);

            CAN_ASCEND = true;
        }
    }

    extern "C" fn handle_ascension_error(_: *mut Display, e: *mut XErrorEvent) -> i32 {
        unsafe {
            if BadAccess == (*e).error_code {
                CAN_ASCEND = false;
            }
            0
        }
    }
}