
use crate::girep::usettings::structs::Usettings;
use crate::commands::core::utils::repo_struct::unfold_repo_structure;


pub(crate) fn validate_repo_structure(value: &str) -> Result<String, String> {
    let usettings = match Usettings::read() {
        Ok(settings) => settings,
        Err(e) => {
            let mut error_message = vec![e.message];
            error_message.extend(e.content);
            return Err(error_message.join("\n"));
        }
    };
    
    let _ = unfold_repo_structure(value, false, &usettings)?;

    Ok(value.to_string())
}

#[allow(dead_code)]
pub(crate) fn validate_repo_structure_with_pconf(value: &str) -> Result<String, String> {
    let usettings = match Usettings::read() {
        Ok(settings) => settings,
        Err(e) => {
            let mut error_message = vec![e.message];
            error_message.extend(e.content);
            return Err(error_message.join("\n"));
        }
    };
    
    let _ = unfold_repo_structure(value, true, &usettings)?;

    Ok(value.to_string())
}
