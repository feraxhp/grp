use grp_core::structs::Comment;
use color_print::cformat;
use grp_core::Platform;
use grp_core::Config;


use super::super::humanize::Humanize;
use super::super::stylizer::Stylizer;
use super::super::FormatTools;
use super::replace_user_mentions;
use super::Show;


impl Show for Vec<Comment> {
    type Context = (Platform, Config);
    
    fn to_string_iter(&self, context: Self::Context) -> impl Iterator<Item = String> + '_ {
        let (platform, config) = context;
        
        (!self.is_empty())
            .then(move || {
                self.iter().map(move |comment| {
                    let body = replace_user_mentions(&comment.body, &platform, &config);
                    cformat!("<dim>--</>\n<g>{}</> <dim>#{} - {}</> \n{}\n\n", comment.author.to_link(), comment.id.as_link(&comment.url), &comment.created_at.to_human(), body)
                })
            })
            .into_iter()
            .flatten()
    }
}
