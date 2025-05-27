use std::time::Duration;
use color_print::{cformat, cprintln};
use indicatif::{ProgressBar, ProgressStyle};
use crate::girep::animations::animation::Animation;

pub(crate) struct Delete {
    spinner: ProgressBar,
}

impl Animation for Delete {
    fn new(message: &str) -> Box<Delete> {
        let spinner = ProgressBar::new_spinner();
        let style = ProgressStyle::default_spinner()
            .tick_strings(
                &[
                    cformat!("<y>♼     🌎</>").as_str(),
                    cformat!("<y>♽➩    🌎</>").as_str(),
                    cformat!("<y>♼ ➪   🌎</>").as_str(),
                    cformat!("<y>♽  ➫  🌎</>").as_str(),
                    cformat!("<y>♼   ➬ 🌎</>").as_str(),
                    cformat!("<y>♽    ➩🌎</>").as_str(),
                ]
            )
            ;
        spinner.set_style(style);
        spinner.set_message(cformat!("<y>{}</>", message.to_string()).to_string());
        spinner.enable_steady_tick(Duration::from_millis(200));

        Box::from(Delete { spinner })
    }

    fn finish_with_error(&self, message: &str) {
        self.spinner.finish_and_clear();
        cprintln!("<r>(--⚡--) {}</>", message.to_string());
    }

    fn finish_with_warning(&self, message: &str) {
        self.spinner.finish_and_clear();
        cprintln!("<y>💻--!--🌎 {}</>", message.to_string());
    }

    fn finish_with_success(&self, message: &str) {
        self.spinner.finish_and_clear();
        cprintln!("<g>(--✻--) {}</>", message.to_string());
    }

    fn change_message(&self, message: String) {
        self.spinner.set_message(cformat!("<y>{}</>", message));
    }
}
