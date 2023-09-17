mod app_window;

use x11::xlib::*;
use x11::keysym::*;
use app_window::AppWindow;
use std::collections::HashMap;


/// The basic Error type of fpwm
pub type Error = &'static str;

/// Used to detect if there is another window manager already running
pub static mut CAN_ASCEND: bool = true;

/// Used to determine whether or not fpwm should be running
pub static mut RUNNING: bool = true;


pub struct WM {
    display: *mut Display,
    root: Window,
    
    current_workspace: usize,
    windows: HashMap<Window, AppWindow>,
    workspaces: HashMap<usize, Vec<Window>>
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
            root,
            
            current_workspace: 1,
            windows: HashMap::new(),
            workspaces: HashMap::new()
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
                XChangeProperty(
                    self.display,
                    self.root,
                    XInternAtom(
                        self.display,
                        "_NET_WM_NAME\0".as_ptr() as *const i8,
                        0
                    ),
                    XInternAtom(
                        self.display, 
                        "UTF8_STRING\0".as_ptr() as *const i8,
                        0
                    ),
                    8,
                    PropModeReplace,
                    "fpwm\0".as_ptr() as *const u8,
                    "fpwm\0".as_bytes().len() as i32,
                );
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
        self.grab_key(self.root, "1", Mod1Mask);
        self.grab_key(self.root, "2", Mod1Mask);

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

    pub fn run(&mut self) {
        unsafe {

            let mut e: XEvent = std::mem::zeroed();
            while RUNNING {

                XNextEvent(self.display, &mut e);

                #[allow(non_upper_case_globals)]
                match e.get_type() {

                    KeyPress => self.handle_keypress(&e.key),
                    CreateNotify => self.handle_create_notify(&e.create_window),
                    DestroyNotify => {

                        self.handle_destroy_notify(&e.destroy_window);

                    },
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

    fn handle_keypress(&mut self, e: &XKeyEvent) {
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
                XK_1 => {

                    if e.state &Mod1Mask == Mod1Mask {

                        self.current_workspace = 1;

                        // turn into function call
                        self.windows.iter_mut().for_each(|w| {

                            if w.1.workspace_id == self.current_workspace {

                                XMapWindow(self.display, w.1.window);

                            }
                            else {

                                XUnmapWindow(self.display, w.1.window);

                            }

                        });

                    }

                }
                XK_2 => {

                    if e.state &Mod1Mask == Mod1Mask {

                        self.current_workspace = 2;
                        self.windows.iter_mut().for_each(|w| {

                            if w.1.workspace_id == self.current_workspace {

                                XMapWindow(self.display, w.1.window);

                            }
                            else {

                                XUnmapWindow(self.display, w.1.window);

                            }

                        });

                    }

                }
                _ => ()

            };

        }
    }

}

// Window creation, destruction, and configuration
impl WM {

    fn handle_create_notify(&mut self, e: &XCreateWindowEvent) {
        
        self.windows.insert(
            e.window,
            AppWindow::new(e.window, self.current_workspace)
        );

        if !self.workspaces.contains_key(&self.current_workspace) {

            self.workspaces.insert(self.current_workspace, vec![]);

        }

        // self.workspaces.get_mut(&self.current_workspace)
        //     .unwrap()
        //     .push(e.window);

    }

    fn handle_destroy_notify(&mut self, e: &XDestroyWindowEvent) {

        self.windows.remove(&e.window);
        
        if !self.workspaces.contains_key(&self.current_workspace) {

            self.workspaces.insert(self.current_workspace, vec![]);

        }

        // remove window from workspace vec

    }

    fn handle_configure_request(&self, e: &XConfigureEvent) {
        unsafe {

            let mut changes = XWindowChanges {
                x: 0,
                y: 0,
                width: 1280,
                height: 800,
                border_width: 0,
                sibling: 0,
                stack_mode: Above
            };

            XConfigureWindow(
                e.display,
                e.window,
                {
                    CWX |
                    CWY |
                    CWWidth |
                    CWHeight |
                    CWBorderWidth |
                    CWStackMode
                } as u32,
                &mut changes
            );
        
        }
    }

    fn handle_map_request(&mut self, e: &XMapRequestEvent) {
        unsafe {

            XMapWindow(self.display, e.window);

        }
    }

}