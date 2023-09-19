// Flying Parrot Window Manager
// 
// Author: DeveloperGY
// License: GPL3
// 
// fpwm is a basic X11 window manager written in rust

mod wm;

use wm::*;

fn main() {
    let mut wm = WM::create().unwrap();
    wm.run();
}

// make a logger