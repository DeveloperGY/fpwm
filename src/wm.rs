use x11::xlib::*;
use x11::keysym::*;

/// The use of global state is required due to how xlib is implemented

/// Represents whether or not there is another window manager running, which
/// would prevent fpwm from becoming the current window manager
static mut CAN_ASCEND: bool = true;


/// The core of fpwm. It represents an X11 Window manger.
pub struct WM {
    /// Main connection to the X server
    display: *mut Display,

    /// The root window
    root: u64
}

/// functions used to create the window manager
impl WM {
    fn new(display: *mut Display) -> Self {

        Self {
            display,
            root: unsafe {XDefaultRootWindow(display)}
        }

    }

    /// Creates the window manager and ascends it
    pub fn create() -> Self {

        let display = match WM::connect() {
            Ok(d) => d,
            Err(e) => panic!("{}", e)
        };

        let wm = WM::new(display);

        match wm.verify_wm_ascension() {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        };

        wm

    }
}

/// Helpers for creating the window manager
impl WM {
    /// Creates a connection to the X server
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

    /// Verify that there is no other window manager currently in use
    fn verify_wm_ascension(&self) -> Result<(), &'static str> {
        unsafe {

            XSetErrorHandler(Some(WM::handle_wm_ascension_error));
            XSelectInput(
                self.display,
                self.root,
                SubstructureRedirectMask | SubstructureNotifyMask
            );
            XSync(self.display, 0);
            XSetErrorHandler(None); // TODO: Create an error handler
        
            if CAN_ASCEND {
                Ok(())
            }
            else {
                Err("Failed to ascend, another window manager is running")
            }
        
        }
    }

    /// Error Handler for if there is another window manager in use
    extern "C" fn handle_wm_ascension_error(

        display: *mut Display,
        e: *mut XErrorEvent
    
    ) -> i32 {
        unsafe {

            match (*e).error_code {
                BadAccess => {
                    CAN_ASCEND = false;
                }
                _ => ()
            };

            0
        
        }
    }
}

/// Handlers for the execution of the window manager
impl WM {
    pub fn run(&self) {
        unsafe {
        
            self.init();
            let mut e: XEvent = std::mem::zeroed();

            loop {
                XNextEvent(self.display, &mut e);

                match e.get_type() {
                    KeyPress => {
                        let key = XLookupKeysym(&mut e.key, 0);

                        if key == XK_Escape as u64 && e.key.subwindow == 0 {
                            break;
                        }
                        else if key == XK_d as u64 && e.key.subwindow == 0 {
                            let mut rofi = std::process::Command::new("rofi");
                            rofi.args(["-show", "window"]);
                            rofi.spawn().unwrap();
                        }
                    },
                    _ => ()
                };
            }
        
            self.clean();

        }
    }

    /// Prepares the window manager to be run
    /// - Grabs input for the window manager
    fn init(&self) {
        self.grab_input(self.display, self.root);
    }

    fn grab_input(&self, display: *mut Display, window: u64) { // TODO: Clean this up
        unsafe {

            let escape = match self.string_to_keycode(display, "Escape\0") {
                Some(c) => c as i32,
                None => panic!("Failed to grab input, invalid key string")
            };

            XGrabKey(
                display,
                escape,
                Mod1Mask, 
                window, 
                1, 
                GrabModeAsync,
                GrabModeAsync
            );

            let d = match self.string_to_keycode(display, "d\0") {
                Some(c) => c as i32,
                None => panic!("Failed to grab input, invalid key string")
            };

            XGrabKey(
                display,
                d,
                Mod1Mask, 
                window, 
                1, 
                GrabModeAsync,
                GrabModeAsync
            );
        }
    }

    fn string_to_keycode(&self, display: *mut Display, key: &str) -> Option<KeyCode> {
        unsafe {

            let key_sym = XStringToKeysym(key.as_ptr() as *mut i8);

            if key_sym == NoSymbol as u64 {
                return None;
            }

            let key_code = XKeysymToKeycode(display, key_sym);

            if key_code == 0 {
                None
            }
            else {
                Some(key_code)
            }
        
        }
    }
}

/// Functions used for cleaning up the window manager
impl WM {
    fn clean(&self) {
        unsafe {

            let escape = match self.string_to_keycode(self.display, "Escape\0") {
                Some(c) => c as i32,
                None => panic!("Failed to grab input, invalid key string")
            };

            XUngrabKey(
                self.display,
                escape,
                Mod1Mask,
                self.root
            );

            XCloseDisplay(self.display);
        
        }
    }
}