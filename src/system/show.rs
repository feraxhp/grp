use color_print::cformat;

use grp_core::structs::Repo;
use grp_core::structs::User;
use grp_core::Error;

use crate::usettings::structs::Pconf;


pub trait Show {
    fn print_pretty(&self) {
        self.to_string_iter().for_each(|s| {
            println!("{}", s)
        });
    }
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_;
}

impl Show for Vec<Repo> {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        let (max_path, max_url) = self.into_iter().fold((4, 3), |(p, u), repo| {
            (p.max(repo.path.len()), u.max(repo.url.len()))
        });
    
        (!self.is_empty())
            .then(move || {
                let header = format!(
                    "{:<width_path$}  {:<5}  {:<width_url$}",
                    "PATH",
                    "STATE",
                    "URL",
                    width_path = max_path,
                    width_url = max_url
                );
    
                let body = self.into_iter().map(move |repo| {
                    let state = match repo.private {
                        Some(true)  => cformat!("<r>priv </>"),
                        Some(false) => cformat!("<g>pub  </>"),
                        None        => cformat!("<y>local</>"),
                    };
                    format!(
                        "{:<width_path$}  {:<5}  {:<width_url$}",
                        repo.path,
                        state,
                        repo.url,
                        width_path = max_path,
                        width_url = max_url
                    )
                });
    
                std::iter::once(header).chain(body)
            })
            .into_iter()
            .flatten()
    }
}

impl Show for Vec<User> {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        self.first()
            .map(|first| {
                let header = match first.path {
                    Some(_) => "PATH".to_string(),
                    None => "NAME".to_string(),
                };
    
                let body = self.into_iter().map(|user| {
                    match &user.path {
                        Some(path) => path.to_string(),
                        None => user.name.to_string(),
                    }
                });
    
                std::iter::once(header).chain(body)
            })
            .into_iter()
            .flatten()
    }
}

impl Show for Vec<Error> {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        self.iter()
            .enumerate()
            .flat_map(|(i, error)| {
                let idx = i + 1;
                let len = idx.to_string().len() + 2;
                
                let header = cformat!("<r>{}: {}</>", idx, error.message);
                let detail = error.to_string_iter(&(len.clone())).collect();
                let blank = String::new();
                
                [header, detail, blank]
            })
    }
}

impl Show for Vec<Pconf> {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        let (max_name, max_endpoint, max_user) = self.into_iter().fold((4, 8, 7), |(n, e, u), pconf| {
            (n.max(pconf.name.len()), e.max(pconf.endpoint.len()), u.max(pconf.owner.len()))
        });
    
        (!self.is_empty())
            .then(move || {
                let header = format!(
                    "{:<max_name$}	{:<max_user$}	{:<max_endpoint$}",
                    "NAME", "USER", "ENDPOINT",
                );
    
                let body = self.into_iter().map(move |pconf| {
                    let name = match pconf.encripted {
                        true  => cformat!("<g>{:<max_name$}</>", &pconf.name),
                        false => cformat!("<r>{:<max_name$}</>", &pconf.name),
                    };
                    
                    format!(
                        "{}	{:<max_user$}	{:<max_endpoint$}",
                        name, pconf.owner.empty(), pconf.endpoint.empty()
                    )
                });
    
                std::iter::once(header).chain(body)
            })
            .into_iter()
            .flatten()
    }
}

trait Empty {
    fn empty(&self) -> String;
}

impl Empty for String {
    fn empty(&self) -> String {
        match self.len() {
            0 => cformat!("<dim,i><<empty>></dim,i>"),
            _ => self.to_string(),
        }
    }
}
