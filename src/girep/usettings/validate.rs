use color_print::cformat;

use super::structs::Usettings;

fn validate(value: &str, vec: Vec<&str>) -> Result<String, String> {
    let config = Usettings::read()
        .map_err(|e| {
            cformat!(
                "{}\n{}",
                e.message,
                e.content.join("\n")
            )
        })?;
    
    let repos = config.pconfs;
    let mut names: Vec<String> = repos.iter().map(|repo| repo.name.clone()).collect();
    let mut add = vec.iter().map(|repo| repo.to_string()).collect();
    names.append(&mut add);
    if names.contains(&value.to_string()) { Ok(value.to_string()) } 
    else {Err(format!(
                "{} is not a valid pconf name\n\
                    possible values are {:?}",
                value, names
    ))}
}

pub fn valid_pconfs(value: &str) -> Result<String, String> {
    validate(value, vec![])
}

pub fn valid_pconfs_and_plus(value: &str) -> Result<String, String> {
    validate(value, vec!["-"])
}