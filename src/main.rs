// Flying Parrot Window Manager
// 
// Author: DeveloperGY
// License: GPL3
// 
// fpwm is a basic X11 window manager written in rust

mod wm;

use wm::*;

fn main() {
    let mut wm = match WM::create() {
        Ok(wm) => wm,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    wm.run();
}

// make a logger (store log in /var/log/fpwm.log)