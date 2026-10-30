use color_print::cformat;
use grp_core::structs::Repo;

use super::Show;


impl Show for Vec<Repo> {
    type Context = ();
    
    fn to_string_iter(&self, _: Self::Context) -> impl Iterator<Item = String> + '_ {
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
