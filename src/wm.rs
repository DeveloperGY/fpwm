mod app_window;
mod initialization;

use x11::xlib::*;
use x11::keysym::*;
use app_window::AppWindow;
use std::collections::HashMap;

/// The Error type of fpwm
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
                    KeyPress         => self.handle_keypress(&e.key),
                    CreateNotify     => self.handle_create_notify(&e.create_window),
                    MapRequest       => self.handle_map_request(&e.map_request),
                    DestroyNotify    => self.handle_destroy_notify(&e.destroy_window),
                    ConfigureRequest => self.handle_configure_request(&e.configure),
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
                        self.switch_workspace(1);
                    }
                }
                XK_2 => {
                    if e.state &Mod1Mask == Mod1Mask {
                        self.switch_workspace(2);
                    }
                }
                XK_3 => {
                    if e.state &Mod1Mask == Mod1Mask {
                        self.switch_workspace(3);
                    }
                }
                XK_4 => {
                    if e.state &Mod1Mask == Mod1Mask {
                        self.switch_workspace(4);
                    }
                }
                XK_5 => {
                    if e.state &Mod1Mask == Mod1Mask {
                        self.switch_workspace(5);
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
    }

    fn handle_destroy_notify(&mut self, e: &XDestroyWindowEvent) {
        self.windows.remove(&e.window);
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

// Workspace Management
impl WM {
    
    fn switch_workspace(&mut self, workspace_id: usize) {
        unsafe {
            self.current_workspace = workspace_id;
            
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
    
}