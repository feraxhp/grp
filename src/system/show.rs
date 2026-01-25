use color_print::cformat;
use color_print::ceprintln;

use grp_core::common::structs::Repo;
use grp_core::common::users::structs::User;
use grp_core::error::structs::Error;

use crate::system::stdout;

pub trait Show {
    fn print_pretty(&self);
}

impl Show for Vec<Repo> {
    fn print_pretty(&self) {
        if self.is_empty() { return; }

        let mut max_path = 4;
        let mut max_url = 3;

        for repo in self {
            max_path = max_path.max(repo.path.len());
            max_url = max_url.max(repo.url.len());
        }

        let header = format!(
            "{:<width_path$}  {:<5}  {:<width_url$}",
            "PATH",
            "STATE",
            "URL",
            width_path = max_path,
            width_url = max_url
        );
        eprintln!("{}", header);

        for repo in self {
            let state = match repo.private {
                Some(true)  => cformat!("<r>priv </>"),
                Some(false) => cformat!("<g>pub  </>"),
                None =>        cformat!("<y>local</>"),
            };
            let line = format!(
                "{:<width_path$}  {:<5}  {:<width_url$}",
                repo.path,
                state,
                repo.url,
                width_path = max_path,
                width_url = max_url
            );
            stdout::writeln(line);
        }
    }
}

impl Show for Vec<User> {
    fn print_pretty(&self) {
        if self.is_empty() { return; }
        
        match &self[0].path {
            Some(_) => eprintln!("PATH"),
            None => eprintln!("NAME"),
        };
        
        for user in self {
            let line = match &user.path {
                Some(path) => format!("{}", path),
                None => format!("{}", user.name),
            };
            stdout::writeln(line);
        }
    }
}

impl Show for Vec<Error> {
    fn print_pretty(&self) {
        if self.is_empty() { return; }
        
        for (i, error) in self.iter().enumerate() {
            let len = (i+1).to_string().len();
            ceprintln!("<r>{}: {}</>", i+1, error.message);
            error.show_with_offset(len+2);
            ceprintln!();
        }
    }
}