use std::time::Duration;
use color_print::{cformat, ceprintln};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use crate::animations::animation::{Create, Style, Subprogress};
use crate::girep::animation::Animation;

impl Style for Create {
    fn normal() -> (ProgressStyle, u64) {
        (
            ProgressStyle::default_spinner()
                .tick_strings(
                    &[
                        cformat!("<y>💻     🌎</>").as_str(),
                        cformat!("<y>💻>    🌎</>").as_str(),
                        cformat!("<y>💻=>   🌎</>").as_str(),
                        cformat!("<y>💻 =>  🌎</>").as_str(),
                        cformat!("<y>💻  => 🌎</>").as_str(),
                        cformat!("<y>💻   =>🌎</>").as_str(),
                        cformat!("<y>💻    =🌎</>").as_str(),
                    ]
                )
            ,
            200
        )
    }
}

impl Animation for Create  {
    fn new<T: Into<String>>(message: T) -> Box<Self> {
        let multi = MultiProgress::new();
        let spinner = multi.add(ProgressBar::new_spinner());
        let style = Self::normal();

        spinner.set_style(style.0);
        spinner.set_message(cformat!("<y>{}</>", message.into()));
        spinner.enable_steady_tick(Duration::from_millis(style.1));

        Box::from(Self { multi, spinners: vec![spinner] })
    }

    fn finish_with_error<T: Into<String>>(&self, message: T) {
        self.finish_all();
        ceprintln!("<r>💻--X--🌎 {}</>", message.into());
    }

    fn finish_with_warning<T: Into<String>>(&self, message: T) {
        self.finish_all();
        ceprintln!("<y>💻--!--🌎 {}</>", message.into());
    }

    fn finish_with_success<T: Into<String>>(&self, message: T) {
        self.finish_all();
        ceprintln!("<g>💻--✓--🌎 {}</>", message.into());
    }

    fn change_message<T: Into<String>>(&self, message: T) {
        self.spinners[0].set_message(cformat!("<y>{}</>", message.into()));
    }
}

impl Subprogress for Create {
    fn add(&mut self) -> usize {
        let pb = self.multi.add(ProgressBar::new(0));
        
        let index = self.spinners.len();
        self.spinners.push(pb);
        index
    }

    fn set_total(&self, index: usize, total: u64, template: &str) {
        let style = Self::progress(template);
        self.spinners[index].set_style(style);
        self.spinners[index].set_length(total);
    }

    fn set_state(&self, index: usize, current: u64) { self.spinners[index].set_position(current) }

    fn set_message<T: Into<String>>(&self, index: usize, message: T) {
        self.spinners[index].set_message(message.into());
    }

    fn finish_all(&self) {
        let len = self.spinners.len();
        for index in 0..len {
            self.spinners[index].finish_and_clear();
        }
    }
}
