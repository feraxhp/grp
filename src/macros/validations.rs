use crate::config::loader::load_configurations;

pub(crate) fn valid_pconfs() -> fn(&str) -> Result<String, String> {
    |value: &str| {
        let config = load_configurations();
        let repos = config.get_repos();
        let names: Vec<String> = repos.iter().map(|repo| repo.name.clone()).collect();
        if names.contains(&value.to_string()) {
            Ok(value.to_string())
        } else {
            Err(
                format!(
                    "{} is not a valid pconf name\n\
                    posible values are {:?}",
                    value, names
                )
            )
        }
    }
}