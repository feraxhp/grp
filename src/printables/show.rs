use color_print::cformat;

use grp_core::structs::Comment;
use grp_core::structs::Issue;
use grp_core::structs::Repo;
use grp_core::structs::User;

use crate::printables::FormatTools;
use crate::printables::humanize::Humanize;
use crate::printables::stylizer::Stylizer;
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
        let (max_path, max_url) = self.iter().fold((4, 3), |(p, u), repo| {
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
    
                let body = self.iter().map(move |repo| {
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
    
                let body = self.iter().map(|user| {
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

impl Show for Vec<Pconf> {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        let (max_name, max_endpoint, max_user) = self.iter().fold((4, 8, 7), |(n, e, u), pconf| {
            (n.max(pconf.name.len()), e.max(pconf.endpoint.len()), u.max(pconf.owner.len()))
        });
    
        (!self.is_empty())
            .then(move || {
                let header = format!(
                    "{:<max_name$}	{:<max_user$}	{:<max_endpoint$}",
                    "NAME", "USER", "ENDPOINT",
                );
    
                let body = self.iter().map(move |pconf| {
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

pub trait Empty {
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

impl Empty for &String {
    fn empty(&self) -> String {
        match self.len() {
            0 => cformat!("<dim,i><<empty>></dim,i>"),
            _ => self.to_string(),
        }
    }
}

impl Show for Vec<Issue> {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        let (max_number, max_autor) = self.iter().fold((2, 6), |(number, author), issue| {
            (
                number.max(issue.number.to_string().len() + 1),
                author.max(issue.author.name.to_string().len()),
            )
        });
    
        (!self.is_empty())
            .then(move || {
                let header = format!(
                    "{:<max_number$}  {:<max_autor$}  {}",
                    "ID", "AUTHOR", "TITLE",
                );
    
                let body = self.iter().map(move |issue| {
                    format!(
                        "{:<max_number$}  {:<max_autor$}  {}",
                        format!("#{}", issue.number).as_link(&issue.url),
                        issue.author.to_link(),
                        issue.title,
                    )
                });
                
                std::iter::once(header).chain(body)
            })
            .into_iter()
            .flatten()
    }
}

impl Show for Vec<Comment> {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        (!self.is_empty())
            .then(move || {
                self.iter().map(move |comment| {
                    cformat!("<dim>--</>\n<g>{}</> <dim>#{} - {}</> \n{}\n\n", comment.author.to_link(), comment.id.as_link(&comment.url), &comment.created_at.to_human(), &comment.body )
                })
            })
            .into_iter()
            .flatten()
    }
}

impl Show for Vec<String> {
    fn to_string_iter(&self) -> impl Iterator<Item = String> + '_ {
        self.iter().map(|s| s.to_owned())
    }
}
