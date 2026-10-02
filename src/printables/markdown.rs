use color_print::cformat;
use grp_core::{Error, empty_notes};
use ink_md::{Args, render::plain::render_plain};


pub trait Markdown {
    fn parse(&self) -> Result<String, Error>;
}

impl Markdown for String {
    fn parse(&self) -> Result<String, Error> { // temporal implementation for the markdown rich terminal output
        let args = Args {
            inputs: vec![],
            theme: "auto".to_string(),
            width: Some(80),
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
        
        render_plain(self, &args).map_err(|e| {
            Error::new(
                "grp::markdown::ink_md", 
                "The markdown parsing whent wrong", 
                cformat!("<y>{e}</>"), 
                vec![],
                empty_notes!()
            )
        })
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
