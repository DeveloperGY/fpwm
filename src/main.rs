/// Flying Parrot Window Manager
/// 
/// Author: DeveloperGY
/// License: GPL3
/// 
/// fpwm is a basic X11 window manager written in rust

mod wm;

use wm::WM;

fn main() {
    let wm = WM::create();
    wm.run();
}