use x11::xlib::*;
use super::super::super::Error;
use std::collections::{HashMap, HashSet};
use super::Keybind;

pub struct Config {
    vars: HashMap<String, String>,
    pub keybinds: HashSet<Keybind>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            vars: HashMap::new(),
            keybinds: HashSet::new()
        }
    }
}

impl Config {
    pub fn parse(config_str: &str) -> Result<Self, Error> {
        let mut config = Config::default();

        let mut line_number: usize = 1;

        for line in config_str.lines().collect::<Vec<_>>() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                line_number += 1;
                continue;
            }

            if let Err(e) = config.configure_line(line) {
                return Err(e + format!(" on line {line_number}").as_str());
            }

            line_number += 1;
        }

        Ok(config)
    }

    /// Parses a single configuration command
    fn configure_line(&mut self, line: &str) -> Result<(), Error> {
        let words = line.split_whitespace().collect::<Vec<_>>();

        let command = words.get(0).unwrap();

        match *command {
            "set" => {
                self.set(&words[1..])?;
                Ok(())
            },
            "bind" => {
                self.bind(&words[1..])?;
                Ok(())
            }
            _ => {
                Err(format!("Invalid Command: \"{}\"", command))
            }
        }
    }

    fn set(&mut self, args: &[&str]) -> Result<(), Error> {
        if args.len() != 2 {
            return Err("Invalid argument count (requires 2)".into());
        }

        let name = args[0].to_string();
        let mut value = args[1].to_string();

        if value.starts_with('$') {
            value = self.get_var(&name)?;
        }

        self.vars.insert(name, value);

        Ok(())
    }

    fn bind(&mut self, args: &[&str]) -> Result<(), Error> {
        if args.len() < 2 {
            return Err("Invalid argument count (requires >=2)".into());
        }

        // arg 1 is the bind
        let (key, modifiers) = self.verify_bind(args[0])?;

        // arg 2.. is the command

        let command = args[1..].join(" ");

        if args[1] == "exec" && args.len() == 2 {
            return Err("Invalid argument count".into());
        }

        let keybind = Keybind::new(&key, modifiers, &command);

        if !self.keybinds.insert(keybind) {
            Err("Key combination already bound".into())
        }
        else {
            Ok(())
        }
    }
}

impl Config {
    /// takes in a variable identifier and returns its value
    pub fn get_var(&self, var: &str) -> Result<String, Error> {
        let name = if var.starts_with('$') {
            var.strip_prefix('$').unwrap()
        }
        else {
            return Ok(var.to_string())
        };
        
        match self.vars.get(name) {
            Some(v) => Ok(v.to_string()),
            None => Err("Invalid variable".into())
        }
    }

    /// Verifies and extracts keybing information from a [`&str`]
    fn verify_bind(&self, bind: &str) -> Result<(String, u32), Error> {
        let keys = bind.split('+').collect::<Vec<_>>();

        let mut expanded_keys = vec![];

        for key in keys {
            let expanded_key = self.get_var(key)?;
            expanded_keys.push(expanded_key.to_lowercase());
        }

        
        let mut has_main_key = false;
        let mut main_key = String::new();
        let mut modifier = 0;

        for key in &expanded_keys {
            if key.len() == 1 { // chnage to handle multi character keys like Esc
                if has_main_key {
                    return Err("Invalid Keybind".into());
                }
                else {
                    main_key = key.clone();
                    has_main_key = true;
                }
            }
            else {
                modifier |= match key.as_str() {
                    "mod1" | "alt"     => Mod1Mask,
                    "mod2"             => Mod2Mask,
                    "mod3"             => Mod3Mask,
                    "mod4"             => Mod4Mask,
                    "mod5"             => Mod5Mask,
                    "shift"            => ShiftMask,
                    "ctrl" | "control" => ControlMask,
                    k => { // Catching any key that is more than 1 character Ex: Escape
                        if has_main_key { // Error out if there is already a main key
                            return Err("Invalid Keybind".into());
                        }
                        else {
                            match k {
                                "escape" => {
                                    main_key = "Escape".into();
                                    has_main_key = true;
                                    0
                                },
                                "enter" | "return" => {
                                    main_key = "Return".into();
                                    has_main_key = true;
                                    0
                                },
                                _ => {
                                    return Err("Invalid Keybind".into());
                                }
                            }
                        }
                    }
                };
            }
        }

        if !has_main_key {
            Err("Invalid Keybind".into())
        }
        else {
            Ok((main_key, modifier))
        }
    }
}