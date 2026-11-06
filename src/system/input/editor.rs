use std::env;
use std::path::PathBuf;
use std::process::Command;

use grp_core::animation::Animation;
use mynox::Context;
use mynox::Error;

use crate::animations::animation::Suspend;
use crate::local::git::utils::GitUtils;
use crate::system::directories::Directories;
use crate::system::directories::Tmp;
use crate::usettings::structs::Usettings;
use crate::system::input::Input;
use crate::system::file::File;


impl Input {
    pub fn use_editor(base_text: &str, name: &str, usettings: &Usettings, repo: Option<&GitUtils>, animation: &Box<impl Suspend + Animation + ?Sized>) -> Result<String, Error>{
        let editor = Self::autodetect_default_editor(usettings, repo)?;
        let file = Self::set_file(base_text, name, repo)?;
        
        animation.change_message("Waiting for the editor to close the file");
        animation.suspend(|| { Self::open_file_in(&editor, &file) })?;
        
        let string = File::read(file.as_path())?;
        Ok(string)
    }
    
    fn autodetect_default_editor(_usettings: &Usettings, repo: Option<&GitUtils>) -> Result<String, Error> {
        // TODO: add editor option on ussetings (not sure, if needed)
        
        if let Ok(editor) = env::var("GIT_EDITOR") { return Ok(editor) };
        
        if let Some(repo) = repo {
            let config = repo.config();
            
            if let Ok(cfg) = config {
                if let Ok(editor) = cfg.get_string("core.editor") {
                    return Ok(editor);
                }
            }
        }
        
        let config = git2::Config::open_default();
        if let Ok(cfg) = config {
            if let Ok(editor) = cfg.get_string("core.editor") {
                return Ok(editor);
            }
        }
        
        env::var("VISUAL")
            .or_else(|_| env::var("EDITOR"))
            .context_raw(
                "input::editor::unknown", 
                "Imposible to auto detect editor",
                "You have to configure at least one editor",
            )
    }
    
    fn open_file_in(editor: &str, file: &PathBuf) -> Result<(), Error>{
        let mut child = Command::new("sh")
                .arg("-c")
                .arg(format!("{} \"$0\"", editor))
                .arg(file)
                .spawn()
                .context(
                    "io::spawn::editor", 
                    "Imposible to spawn de editor configured"
                )
        ?;
        
        child.wait()
            .context(
                "io::spawn::editor", 
                "This happends while waitgin for the editor to close the file"
            )?;
        
        Ok(())
    }
    
    fn set_file(text: &str, name: &str, repo: Option<&GitUtils>) -> Result<PathBuf, Error> {
        let directory = Tmp::directory()?;
        let path = match repo {
            Some(repo) => {
                let workdir = repo.commondir();
                workdir
            },
            None => {
                Tmp::create_dirs(&directory)?;
                directory.as_path()
            },
        }; 
        
        let path = path.join(name);
        File::write(&path, text)?;
        Ok(path)
    }
}
