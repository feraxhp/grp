use std::{fs::{create_dir_all, File}, path::PathBuf};

use crate::girep::error::{structs::Error, types::ErrorType};

pub struct Directories;

impl Directories {
    #[allow(dead_code)]
    pub fn config_file() -> Result<PathBuf, Error> {
        let path = Self::config_dir()?.join("config.json");
        if !path.exists() {
            File::create(&path)
                .expect("Failed to create config directory");
        }
        Ok(path)
    }
    
    #[allow(dead_code)]
    pub fn config_dir() -> Result<PathBuf, Error> {
        let dir_name = "girep";
        let home_dir = match dirs::home_dir() {
            Some(e) => e,
            None => return Err(
                Error::new(
                    ErrorType::Path404, 
                    vec![
                        "HOME_DIR",
                        "Imposible to optain the home directory"
                    ]
                )
            ),
        };
        let location = match std::env::consts::OS {
            "linux" => home_dir.join(format!(".config/{}", dir_name)),
            "windows" => {
                let appdata = match std::env::var("APPDATA") {
                    Ok(s) => s,
                    Err(_) => return Err(
                        Error::new(
                            ErrorType::Path404, 
                            vec![
                                "APPDATA",
                                "The env variable is not configured!"
                            ]
                        )
                    ),
                };
                PathBuf::from(appdata).join(dir_name)
            },
            "macos" => home_dir.join(format!("Library/Application Support/{}", dir_name)),
            _ => home_dir.join(format!(".config/{}", dir_name)),
        };
    
        // Create file if it does not exist
        if !location.exists() { create_dir_all(&location)
            .expect("Failed to create config directory");
        }
    
        Ok(location)
    }
    
    pub fn current_dir() -> Result<PathBuf, Error> {
        let current_dir = std::env::current_dir()
            .map_err(|e| Error::new(
                ErrorType::Path404, 
                vec![
                    "CURRENT_DIR",
                    "Directory",
                    &e.to_string()
                ]
            ))?;
        
        Ok(current_dir)
    }
}
