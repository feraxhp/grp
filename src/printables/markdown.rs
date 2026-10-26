use color_print::cformat;

use ink_md::render::plain::render_plain;
use ink_md::Width::Columns;
use ink_md::Args;

use mynox::Context;
use mynox::Error;


pub trait Markdown {
    fn parse(&self) -> Result<String, Error>;
}

impl Markdown for String {
    fn parse(&self) -> Result<String, Error> { // temporal implementation for the markdown rich terminal output
        let args = Args {
            inputs: vec![],
            theme: "terminal".to_string(),
            width: Columns(80),
            slides: true,
            plain: true,
            watch: false,
            toc: false,
            images: ink_md::image::ImageMode::Off,
            image_protocol: ink_md::graphics::ProtocolChoice::Auto,
            frontmatter: true,
            spacing: ink_md::Spacing::Normal,
            mouse_capture: false,
            clipboard: ink_md::clipboard::ClipboardMode::Off,
        };

        render_plain(self, &args)
            .context_display("grp::markdown::ink_md", "The markdown parsing whent wrong")
    }
}

impl Markdown for Option<String> {
    fn parse(&self) -> Result<String, Error> {
        match self {
            Some(s) => s.parse(),
            None => Ok(cformat!("<dim, i>empty</>")),
        }
    }
}
