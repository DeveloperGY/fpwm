use super::super::super::Error;
use std::collections::HashMap;

pub struct Config {
    vars: HashMap<String, String>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            vars: HashMap::new()
        }
    }
}

impl Config {
    pub fn parse(config_str: &str) -> Result<Self, Error> {
        let mut config = Config::default();

        let mut line_number: usize = 0;

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
            _ => {
                Err(format!("Invalid Command: \"{}\"", command))
            }
        }
    }

    fn set(&mut self, args: &[&str]) -> Result<(), Error> {
        if args.len() != 2 {
            return Err("Invalid argument count".into());
        }

        let name = args[0].to_string();
        let mut value = args[1].to_string();

        if value.starts_with('$') {
            value = self.get_var(&name)?;
        }

        self.vars.insert(name, value);

        Ok(())
    }
}

impl Config {
    /// takes in a variable identifier and returns its value
    pub fn get_var(&self, var: &str) -> Result<String, Error> {
        let name = if var.starts_with('$') {
            var.strip_prefix('$').unwrap()
        }
        else {
            var
        };
        
        match self.vars.get(name) {
            Some(v) => Ok(v.to_string()),
            None => Err("Invalid variable".into())
        }
    }
}