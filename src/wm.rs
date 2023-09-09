use x11::xlib::*;

/// The use of static state is required due to how xlib is implemented

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
            let mut e: XEvent = std::mem::zeroed();

            loop {
                XNextEvent(self.display, &mut e);

                match e.get_type() {
                    _ => ()
                }
            }
        }

    }
}