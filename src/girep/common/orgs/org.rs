use serde::Deserialize;

#[derive(Clone)]
pub(crate) struct Org {
    name: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Helper {
    Name { name: String, },
    Login { login: String, },
}

impl<'de> Deserialize<'de> for Org {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = Helper::deserialize(deserializer)?;

        let name = match helper {
            Helper::Login { login } => login,
            Helper::Name { name } => name,
        };

        Ok(Org { name })
    }
}

impl Org {
    pub(crate) fn show(orgs: Vec<Org>){
        use std::cmp::max;

        if orgs.is_empty() {
            eprintln!("No repositories found");
            return;
        }

        let mut orgs = orgs.clone();
        orgs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        let length = orgs.len().to_string().len();
        let max_name = orgs.iter().map(|org| org.name.len()).max().unwrap();

        let max_name = max(4, max_name);

        eprintln!(
            "{0:∼^dig$}",
            "", dig = length + max_name + 4,
        );

        for (index, org) in orgs.iter().enumerate() {
            eprintln!(
                "⁞ {0: ^dig$} {1: <width$} ⁞",
                index + 1, org.name,
                width = max_name,
                dig = length,
            );
        }

        eprintln!(
            "{0:∼^dig$}",
            "", dig = length + max_name + 4,
        );
    }
}
