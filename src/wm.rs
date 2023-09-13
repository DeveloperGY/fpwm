use x11::xlib::*;
use x11::keysym::*;
use super::fpwm;



/// Used to detect if there is another window manager already running
static mut CAN_ASCEND: bool = true;

/// Used to determine whether or not the window manager should be running
static mut RUNNING: bool = true;



/// Creates the window manager
pub fn create_window_manager() -> Result<*mut Display, fpwm::Error> {

    let display = connect_to_x_server()?;

    ascend_wm(display)?;   

    Ok(display)

}

/// Configures the window manager
pub fn configure_window_manager(
    display: *mut Display
) -> Result<(), fpwm::Error> {

    grab_wm_inputs(display)?;
    Ok(())

}

/// Runs the window manager
pub fn run_window_manager(display: *mut Display) {
    unsafe {

        let mut e: XEvent = std::mem::zeroed();

        while RUNNING {

            XNextEvent(display, &mut e);

            #[allow(non_upper_case_globals)]
            match e.get_type() {
                
                KeyPress => {

                    handle_key_press(display, &e.key);

                }
                ConfigureRequest => {

                    handle_configure_request(display, &e.configure_request);

                }
                MapRequest => {

                    handle_map_request(display, &e.map_request);

                }
                _ => ()

            };

        }

    }
}

/// Attempts to connect to the X server
/// Retuns a raw pointer to a [`Display`] on success
fn connect_to_x_server() -> Result<*mut Display, fpwm::Error> {
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

/// Attempts to activate the window manager
/// Fails if there is already another window manager running
fn ascend_wm(
    display: *mut Display
) -> Result<(), fpwm::Error> {
    unsafe {

        XSetErrorHandler(Some(handle_wm_ascension_error));
        XSelectInput(
            display,
            XDefaultRootWindow(display),
            SubstructureNotifyMask | SubstructureRedirectMask
        );
        XSync(display, 0);

        if !CAN_ASCEND {
            Err(
                "Failed to ascend window manager, another window manager is \
                running"
            )
        }
        else {
            XSetErrorHandler(None);
            Ok(())
        }

    }
}

/// Handles the error that occurs if another window manager is already running
/// during the ascension process
extern "C" fn handle_wm_ascension_error(
    _display: *mut Display,
    e: *mut XErrorEvent
) -> i32 {
    unsafe {

        #[allow(non_upper_case_globals)]
        match (*e).error_code {

            BadAccess => {

                CAN_ASCEND = false;

            }
            _ => ()

        };

        0

    }
}

/// Grabs the inputs for the window manager
fn grab_wm_inputs(display: *mut Display) -> Result<(), fpwm::Error> {
    unsafe {
        
        let root = XDefaultRootWindow(display);
        grab_wm_keys(display, root)?;
        // get mouse inputs

        Ok(())
    
    }
}

/// Grabs keybinds for the window manager
fn grab_wm_keys(
    display: *mut Display,
    window: Window
) -> Result<(), fpwm::Error> {

    // Grab Alt + Escape
    grab_key(display, window, "Escape", Mod1Mask)?;

    grab_key(display, window, "d", Mod1Mask)?;

    Ok(())

}

/// Grabs a keybind
fn grab_key(
    display: *mut Display,
    window: Window,
    key: &'static str,
    modifiers: u32
) -> Result<(), fpwm::Error> {
    unsafe {

        let keystr = str_to_cstring(key);

        XGrabKey(
            display,
            keystr_to_keycode(display, keystr.as_str())? as i32,
            modifiers,
            window,
            1,
            GrabModeAsync,
            GrabModeAsync
        );

        Ok(())

    }
}

/// Converts a string to its respective keycode. You must include the null
/// terminator character as it is a c string
fn keystr_to_keycode(
    display: *mut Display,
    keystr: &str
) -> Result<u64, fpwm::Error> {
    unsafe {

        let sym = XStringToKeysym(keystr.as_ptr() as *const i8);

        if sym == NoSymbol as u64 {

            return Err("Failed to convert keystr to keysym");
        
        }

        let code = XKeysymToKeycode(display, sym);

        if code == 0 {

            Err("Failed to convert keysym to keycode")
        
        }
        else {
            
            Ok(code as u64)
        
        }

    }
}

/// This takes a rust string and appends a null terminator character
fn str_to_cstring(str: &str) -> String {
    
    let mut c_string = str.to_string();

    if !c_string.ends_with("\0") {
        
        c_string.push('\0');
    
    }

    c_string

}

/// Handles keypress events
fn handle_key_press(display: *mut Display, e: &XKeyEvent) {
    unsafe {

        let keysym = XKeycodeToKeysym(display, e.keycode as u8, 0);

        #[allow(non_upper_case_globals)]
        match keysym as u32 {

            XK_Escape => {

                RUNNING = false;
            
            }

            XK_d => {

                // open rofi

                let mut rofi = std::process::Command::new("rofi");
                rofi.args(["-show", "window"]);
                rofi.spawn().unwrap();

            }

            _ => ()

        };

    }
}

fn handle_configure_request(display: *mut Display, e: &XConfigureRequestEvent) {
    unsafe {

        let mut changes = XWindowChanges {
            x: e.x,
            y: e.y,
            width: e.width,
            height: e.height,
            border_width: e.border_width,
            sibling: 0,
            stack_mode: Above
        };

        XConfigureWindow(display, e.window, e.value_mask as u32, &mut changes);

    }
}

/// Handles window map requests
fn handle_map_request(display: *mut Display, e: &XMapRequestEvent) {
    unsafe {

        XMapWindow(display, e.window);

    } 
}