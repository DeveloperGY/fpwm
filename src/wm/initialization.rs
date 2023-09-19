mod configuration;

use super::{WM, Error, CAN_ASCEND};
use configuration::load_config;
use x11::xlib::*;
use std::collections::HashMap;
use std::ffi::CString;

// Window Manager Initialization
impl WM {
    
    // Creates and validates window manager
    pub fn create() -> Result<Self, Error> {
        let _config = load_config();
        
        let wm = WM::new()?;
        wm.ascend()?;
        wm.grab_input()?; // TODO: Replace with configuration
        Ok(wm)
    }

    /// Creates an instance of an [`WM`]
    fn new() -> Result<Self, Error> {
        let display = WM::connect()?;
        let root = unsafe {XDefaultRootWindow(display)};

        let wm = Self {
            display,
            root,
            
            current_workspace: 1,
            windows: HashMap::new()
        };

        Ok(wm)
    }

    /// Connects to the X server and returns a [`Display`]
    ///
    /// Returns an [`Error`] if it failed to retrieve the [`Display`]
    fn connect() -> Result<*mut Display, Error> {
        unsafe {
            let display = XOpenDisplay(std::ptr::null());

            if display.is_null() {
                Err("Failed to connect to X server, is it running?")
            }
            else {
                Ok(display)
            }
        }
    }

    /// Attempts to ascend the window manager
    ///
    /// Returns [`Error`] if there is another window manager already running
    fn ascend(&self) -> Result<(), Error> {
        unsafe {
            XSetErrorHandler(Some(WM::handle_ascension_error));
            XSelectInput(
                self.display,
                self.root,
                SubstructureNotifyMask | SubstructureRedirectMask
            );
            XSync(self.display, 1);
            XSetErrorHandler(None);

            if !CAN_ASCEND {
                Err("Failed to ascend fpwm, is another window manager already running?")
            }
            else {
                Ok(())
            }
        }
    }

    extern "C" fn handle_ascension_error(_: *mut Display, e: *mut XErrorEvent) -> i32 {
        unsafe {
            if BadAccess == (*e).error_code {CAN_ASCEND = false;}
            0
        }
    }

}

// Input Grabbing
impl WM {

    fn grab_input(&self) -> Result<(), Error> {
        self.grab_keys()?;
        Ok(())
    }

    fn grab_keys(&self) -> Result<(), Error> {
        self.grab_key(self.root, "d", Mod1Mask)?;
        self.grab_key(self.root, "Escape", Mod1Mask)?;
        self.grab_key(self.root, "1", Mod1Mask)?;
        self.grab_key(self.root, "2", Mod1Mask)?;
        self.grab_key(self.root, "3", Mod1Mask)?;
        self.grab_key(self.root, "4", Mod1Mask)?;
        self.grab_key(self.root, "5", Mod1Mask)?;
        Ok(())
    }

    fn grab_key(
        &self,
        window: Window,
        key: &str,
        modifiers: u32
    ) -> Result<(), Error> {
        unsafe {
            let keycode = self.keystr_to_keycode(key)?;
            XGrabKey(
                self.display,
                keycode as i32,
                modifiers,
                window,
                0,
                GrabModeAsync,
                GrabModeAsync
            );
            Ok(())
        }
    }

    fn keystr_to_keycode(&self, key: &str) -> Result<KeyCode, Error> {
        unsafe {
            let keystr = match CString::new(key) {
                Ok(s) => s,
                Err(_) => return Err("Failed to generate keycode, invalid string!")
            };

            let keysym = XStringToKeysym(keystr.as_ptr());
            Ok(XKeysymToKeycode(self.display, keysym))
        }
    }

}