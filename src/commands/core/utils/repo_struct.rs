use color_print::cformat;
use crate::usettings::structs::Usettings;

const SHORT_PATHS: [&str; 2] = [
    "gitea",
    "github",
];

const LONG_PATHS: [&str; 2] = [
    "gitlab",
    "bitbucket",
];

fn manage_long_paths(platform: &String) -> Result<bool, String> {
    if LONG_PATHS.contains(&platform.as_str()) {
        return Ok(true);
    }
    
    if SHORT_PATHS.contains(&platform.as_str()) {
        return Ok(false);
    }
    
    Err(format!("\n* <y>{}</> is not a valid platform", platform))
}

pub(crate) fn unfold_repo_structure(value: &str, required_pconf: bool, usettings: &Usettings) -> Result<(Option<String>, String, String), String> {
    let (pconf, path) = match value.split_once(':') {
        Some((p, r)) => (Some(p.to_string()), r.to_string()),
        None => {
            if required_pconf {
                return Err(
                    cformat!("\n* you have to provide a <g>pconf</>")
                );
            }
            (None, value.to_string())
        }
    };
    
    let conf = match &pconf { 
        Some(pconf_name) if pconf_name != "*" => usettings.get_pconf_by_name(pconf_name.as_str()),
        _ => usettings.get_default_pconf()
    };
    
    let (platform, df_owner) = match conf {
        Some(pconf) => (pconf.r#type, pconf.owner),
        None => {
            return Err(cformat!("\n* No default pconf configured!"));
        }
    };
    
    let parts: Vec<&str> = path.split('/').collect();
    if parts.is_empty() {
        return Err(
            cformat!("\n* the structure must follow <m>[pconf]:<<owner>[/subgroup]/<<repo></>")
        );
    }

    let (owner, repo_name) = match manage_long_paths(&platform)? {
        true => {
            if parts.len() < 2 {
                return Err(
                    cformat!("\n* the structure must follow <m>[pconf]:<<owner>[/subgroup]/<<repo></>")
                );
            }

            if parts[0] == "*" {
                if parts.len() != 2 {
                    return Err(
                        cformat!("\n* when using the default owner, the structure must be <m>[pconf]:*/<<repo></>")
                    );
                }
                ( df_owner, parts[1].to_string() )
            } else {
                let repo = parts.last().unwrap();
                let owner = parts[..parts.len()-1].join("/");
                (owner, repo.to_string())
            }
        },
        false => {
            if parts.len() != 2 {
                return Err(
                    vec![
                        cformat!("\n* the structure must follow <m>[pconf]:<<owner>/<<repo></>"),
                        cformat!("  » the platform: <g>{}</> does not allow subgroups", platform),
                    ].join("\n")
                );
            }
            let owner = if parts[0] == "*" { df_owner } else { parts[0].to_string() };
            (owner, parts[1].to_string())
        }
    };

    Ok((pconf, owner, repo_name))
}

