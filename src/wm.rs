use x11::xlib::*;
use x11::keysym::*;


/// The basic Error type of fpwm
pub type Error = &'static str;

/// Used to detect if there is another window manager already running
pub static mut CAN_ASCEND: bool = true;

/// Used to determine whether or not fpwm should be running
pub static mut RUNNING: bool = true;


pub struct WM {
    display: *mut Display,
    root: Window
}

impl WM {

    pub fn create() -> Result<Self, Error> {

        let wm = WM::new()?;
        wm.ascend()?;
        wm.grab_input();
        Ok(wm)

    }

    fn new() -> Result<Self, Error> {

        let (display, root) = WM::retrieve_x_data()?;

        let wm = Self {
            display,
            root
        };

        Ok(wm)

    }

}

// Window Manager Creation
impl WM {

    fn retrieve_x_data() -> Result<(*mut Display, Window), Error> {
        unsafe {

            let display = WM::connect()?;
            let root = XDefaultRootWindow(display);

            Ok((display, root))

        }
    }

    /// Connects to the X server and returns the display
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

}

// Window Manager Initialization
impl WM {

    fn ascend(&self) -> Result<(), Error> {
        unsafe {

            self.attempt_ascension();

            if !CAN_ASCEND {
                Err(
                    "Failed to ascend fpwm, is another window manager already \
                    running?"
                )
            }
            else {
                Ok(())
            }

        }
    }

    fn attempt_ascension(&self) {
        unsafe {

            XSetErrorHandler(Some(WM::handle_ascension_error));
            XSelectInput(
                self.display,
                self.root,
                SubstructureNotifyMask | SubstructureRedirectMask
            );
            XSync(self.display, 1);
            XSetErrorHandler(None);

        }
    }

    extern "C" fn handle_ascension_error(
        _: *mut Display,
        e: *mut XErrorEvent
    ) -> i32 {
        unsafe {

            if BadAccess == (*e).error_code {CAN_ASCEND = false;}
            0

        }
    }

}

// Input Grabbing
impl WM {

    fn grab_input(&self) {

        self.grab_keys();

    }

    fn grab_keys(&self) {

        self.grab_key(self.root, "d", Mod1Mask);
        self.grab_key(self.root, "Escape", Mod1Mask);

    }

    fn grab_key(&self, window: Window, key: &str, modifiers: u32) {
        unsafe {

            let keycode = self.keystr_to_keycode(key);
            XGrabKey(
                self.display,
                keycode as i32,
                modifiers,
                window,
                0,
                GrabModeAsync,
                GrabModeAsync
            );

        }
    }

    fn keystr_to_keycode(&self, key: &str) -> KeyCode {
        unsafe {

            let keystr = self.str_to_cstring(key);
            let keysym = XStringToKeysym(keystr.as_ptr() as *const i8);
            XKeysymToKeycode(self.display, keysym)

        }
    }

    fn str_to_cstring(&self, str: &str) -> String {

        let mut c_string = str.to_string();

        if !c_string.ends_with("\0") {

            c_string.push('\0');

        }

        c_string

    }

}

// Window Manager Execution
impl WM {

    pub fn run(&self) {
        unsafe {

            let mut e: XEvent = std::mem::zeroed();
            while RUNNING {

                XNextEvent(self.display, &mut e);

                #[allow(non_upper_case_globals)]
                match e.get_type() {

                    KeyPress => self.handle_keypress(&e.key),
                    MapRequest => self.handle_map_request(&e.map_request),
                    ConfigureRequest => {

                        self.handle_configure_request(&e.configure);
                    
                    },
                    _ => ()

                };

            }

        }
    }

}

// Handle Key Presses
impl WM {

    fn handle_keypress(&self, e: &XKeyEvent) {
        unsafe {

            let keysym = XKeycodeToKeysym(
                self.display,
                e.keycode as u8,
                0
            ) as u32;

            #[allow(non_upper_case_globals)]
            match keysym {

                XK_d => {

                    if e.state & Mod1Mask == Mod1Mask {

                        let mut rofi = std::process::Command::new("rofi");
                        rofi.args(["-modi", "drun,run", "-show", "drun"]);
                        rofi.spawn().unwrap();

                    }

                },
                XK_Escape => {

                    if e.state & Mod1Mask == Mod1Mask {RUNNING = false;}

                },
                _ => ()

            };

        }
    }

}

// Window creation, destruction, and configuration
impl WM {

    fn handle_configure_request(&self, e: &XConfigureEvent) {
        unsafe {

            let mut changes = XWindowChanges {
                x: 0,
                y: 0,
                width: 1080,
                height: 720,
                border_width: 0,
                sibling: 0,
                stack_mode: Above
            };

            XConfigureWindow(
                e.display,
                e.window,
                0,
                &mut changes
            );

            // (
            //        CWX | CWY | CWWidth | CWHeight | CWBorderWidth | CWSibling
            //      | CWStackMode
            //  ) as u32,
        
        }
    }

    fn handle_map_request(&self, e: &XMapRequestEvent) {
        unsafe {

            XMapWindow(self.display, e.window);

        }
    }

}