use x11::xlib::*;

#[derive(Clone, Copy)]
pub struct AppWindow {
    pub window: Window,
    pub workspace_id: usize
}

impl AppWindow {
    
    pub fn new(window: Window, workspace_id: usize) -> Self {
        
        Self {
            window,
            workspace_id
        }
        
    }
    
}