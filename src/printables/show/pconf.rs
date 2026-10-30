use color_print::cformat;

use crate::usettings::structs::Pconf;
use super::super::show::Empty;
use super::Show;


impl Show for Vec<Pconf> {
    type Context = ();
    
    fn to_string_iter(&self, _: Self::Context) -> impl Iterator<Item = String> + '_ {
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