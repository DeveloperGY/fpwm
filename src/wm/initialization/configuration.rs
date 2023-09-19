mod config;

use super::super::Error;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use config::Config;

/// Loads the fpwm configuration
///
/// Sources for configuration in order are
///
/// 1. `$XDG_CONFIG_HOME/fpwm/fpwm.conf` (if `$XDG_CONFIG_HOME` is set)
///
///    `$HOME/.config/fpwm/fpwm.conf`    (if `$XDG_CONFIG_HOME` is not set)
///
/// 2. `$XDG_CONFIG_DIRS/fpwm/fpwm.conf` (if `$XDG_CONFIG_DIRS` is set)
///
///    `/etc/fpwm/fpwm.conf`             (if `$XDG_CONFIG_DIRS` is not set)
///
/// If none of these 4 locations have a configuration file, then an [`Error`]
/// is returned
pub fn load_config() -> Result<Config, Error> {
    let mut config_file = get_config_file()?;

    let mut config_string = String::new();

    if let Err(_) = config_file.read_to_string(&mut config_string) {
        return Err("Failed to read configuration file, is it valid UTF-8?".into());
    }

    let config = Config::parse(&config_string)?;

    Ok(config)
}

/// Returns the path to the highest priority configuration file
/// if no configuration file exists, it returns [`Error`]
fn get_config_file() -> Result<File, Error> {
    let relative_path = Path::new("fpwm/fpwm.conf");

    let fail = Err(r#"Failed to find valid configuration file, run "fpwm --generate" to generate the default configuration at /etc/fpwm/fpwm.conf"#.into());

    let mut is_local_path = true; // True if it finds one of the first two paths
    
    let mut path = match get_local_config_path() {
        Ok(p) => {
            p
        },
        Err(_) => {
            is_local_path = false;
            get_global_config_path()
        }
    };

    path.extend(relative_path);

    Ok(match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            if !is_local_path {
                return fail;
            }
            else {
                let mut global_path = get_global_config_path();
                global_path.extend(relative_path);
                match File::open(global_path) {
                    Ok(f) => f,
                    Err(_) => {
                        return fail;
                    }
                }
            }
        }
    })
}

/// Returns the path to the user's default configuration location
fn get_local_config_path() -> Result<PathBuf, Error> {
    let config_home = match std::env::var("XDG_CONFIG_HOME") {
        Ok(p) => PathBuf::from(p),
        Err(_) =>  {
            let mut default_home = match std::env::var("HOME") {
                Ok(p) => PathBuf::from(p),
                Err(_) => {
                    return Err("Failed to get user's HOME directory".into())
                }  
            };

            default_home.extend(Path::new(".config"));
            default_home
        }
    };
    
    Ok(config_home)
}

/// Returns the path to the global default configuration location
fn get_global_config_path() -> PathBuf {
    let config_global = match std::env::var("XDG_CONFIG_DIR") {
        Ok(p) => PathBuf::from(p),
        Err(_) => PathBuf::from("/etc")
    };
    
    config_global
}