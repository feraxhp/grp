use color_print::cformat;
use grp_core::structs::Comment;


use super::super::humanize::Humanize;
use super::super::stylizer::Stylizer;
use super::super::FormatTools;
use super::Show;


impl Show for Vec<Comment> {
    type Context = ();
    
    fn to_string_iter(&self, _: Self::Context) -> impl Iterator<Item = String> + '_ {
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
