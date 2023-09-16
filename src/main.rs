// Flying Parrot Window Manager
// 
// Author: DeveloperGY
// License: GPL3
// 
// fpwm is a basic X11 window manager written in rust


// config directories (ordered)
//
// $XDG_CONFIG_HOME/fpwm/fpwm.conf
// or if $XDG_CONFIG_HOME isn't set
// $HOME/.config/fpwm/fpwm.conf
//
// $XDG_CONFIG_DIRS/fpwm.conf
// or if $XDG_CONFIG_DIRS isn't set
// /etc/fpwm.conf

mod wm;

use wm::*;

fn main() {

    let wm = WM::create().unwrap();
    wm.run();

}