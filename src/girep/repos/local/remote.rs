use std::process::Command;
use std::io::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use clap::builder::Str;
use color_print::cformat;

pub(crate) fn add_remote(remote_url: &str, remote_name: &str, path_buf: PathBuf) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let _ = std::env::set_current_dir(path_buf);
    let has_git_installed = match Command::new("git")
        .arg("--version")
        .output()
    {
        Ok(_) => true,
        Err(_) => false
    };

    if !has_git_installed {
        return Err(Error::new(ErrorKind::NotFound, cformat!("<i>git</> is not installed")));
    }

    let is_git_repository = match Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
    {
        Ok(output) => {
            let output = String::from_utf8(output.stdout).unwrap();
            output.trim().eq("true")
        },
        Err(_) => false
    };

    if !is_git_repository {
        return Err(Error::new(ErrorKind::NotFound, cformat!("This is not a <i>git repository</>")));
    }

    let _ = match Command::new("git")
        .arg("remote")
        .arg("add")
        .arg(remote_name.clone())
        .arg(remote_url)
        .output()
    {
        Ok(_) => {},
        Err(_) => return Err(Error::new(ErrorKind::NotFound, "Failed to add remote".to_string()))
    };

    Ok(())
}