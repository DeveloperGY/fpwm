use super::super::Error;
use std::path::{Path, PathBuf};

/// Loads the fpwm configuration
///
/// Sources for configuration in order are
///
/// 1. `$XDG_CONFIG_HOME/fpwm/fpwm.conf` (if `$XDG_CONFIG_HOME` is set)
///
///    `$HOME/.config/fpwm/fpwm.conf`    (if `$XDG_CONFIG_HOME` is not set)
///
/// 2. `$XDG_CONFIG_DIRS/fpwm.conf` (if `$XDG_CONFIG_DIRS` is set)
///
///    `/etc/fpwm.conf`             (if `$XDG_CONFIG_DIRS` is not set)
///
/// If none of these 4 locations have a configuration file, then an [`Error`]
/// is returned
pub fn load_config() -> Result<(), Error> {
    get_local_config_path()?;
    Ok(())
}

fn get_local_config_path() -> Result<PathBuf, Error> {
    let user_home = match std::env::var("HOME") {
        Ok(p) => p,
        Err(_) => {
            return Err("Failed to get user's HOME directory")
        }  
    };
    
    let config_home = match std::env::var("XDG_CONFIG_HOME") {
        Ok(p) => PathBuf::from(p),
        Err(_) =>  {
            let mut home_path = PathBuf::from(user_home);
            home_path.extend(Path::new(".config"));
            home_path.extend(Path::new("fpwm"));
            home_path
        }
    };

    println!("Config Home: {}", config_home.to_str().unwrap());
    
    Err("")
}