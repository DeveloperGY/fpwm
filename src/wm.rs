mod app_window;
mod initialization;

use x11::xlib::*;
use app_window::AppWindow;
use std::collections::HashMap;
use std::process::Command;
use std::ffi::CString;
use initialization::{Config, Keybind};

/// The Error type of fpwm
pub type Error = String;

/// Used to detect if there is another window manager already running
pub static mut CAN_ASCEND: bool = true;

/// Used to determine whether or not fpwm should be running
pub static mut RUNNING: bool = true;


pub struct WM {
    config: Config,

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

    fn handle_keypress(&mut self, e: &XKeyEvent) { // TODO: refactor to make more understandable
        unsafe {
            let event_keysym = XKeycodeToKeysym(
                self.display,
                e.keycode as u8,
                0
            );

            let mut keybind_found = false;
            let mut selected_keybind = Keybind::new("", 0, ""); // will be overridden if match is found

            for keybind in &self.config.keybinds {
                let keybind_cstring = CString::new(keybind.key.clone()).unwrap();
                let keybind_keysym = XStringToKeysym(keybind_cstring.as_ptr());

                let matches_keysym = event_keysym == keybind_keysym;
                let matches_modifiers = e.state & keybind.modifiers == keybind.modifiers;

                let matches_keybind = matches_keysym && matches_modifiers;

                if matches_keybind {
                    keybind_found = true;
                    selected_keybind = keybind.clone();
                    break;
                }
            }

            if keybind_found {
                self.run_command(&selected_keybind);
            }
        }
    }

    fn run_command(&mut self, keybind: &Keybind) {
        let command = keybind.command.as_str();

        let words = command
            .split_whitespace()
            .collect::<Vec<_>>();

        match words[0] {
            "exit" => {
                unsafe {RUNNING = false};
            }
            "exec" => {
                let mut command = Command::new(words[1]);
                command.args(&words[1..]);
                let _child = command.spawn().unwrap();
                // TODO: add child to child process tracker unless its meant to be seperate
            },
            "workspace" => {
                let workspace_id = match self.config.get_var(words[1]) {
                    Ok(id) => {
                        match id.as_str().parse() {
                            Ok(value) => value,
                            Err(_) => {
                                return; // TODO: return error for logger
                            }
                        }
                    },
                    Err(_) => {
                        return; // TODO: return error for logger
                    }
                };

                self.switch_workspace(workspace_id);
            }
            "move" => {
                let workspace_id: usize = match self.config.get_var(words[1]) {
                    Ok(id) => {
                        match id.as_str().parse() {
                            Ok(value) => value,
                            Err(_) => {
                                return; // TODO: return error for logger
                            }
                        }
                    },
                    Err(_) => {
                        return; // TODO: return error for logger
                    }
                };

                let mut window = 0;
                let mut state = 0;

                unsafe {XGetInputFocus(self.display, &mut window, &mut state)};

                if window != self.root {
                    unsafe {
                        XUnmapWindow(self.display, window);
                        if let Some(app_window) = self.windows.get_mut(&window) {
                            app_window.workspace_id = workspace_id;
                        }
                    }
                }
            }
            _ => () 
        };
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