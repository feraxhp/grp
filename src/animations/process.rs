use std::time::Duration;
use color_print::{cformat, cprintln};
use indicatif::{ProgressBar, ProgressStyle};
use crate::animations::animation::Process;
use crate::girep::animation::Animation;


impl Animation for Process {
    fn new<T: Into<String>>(message: T) -> Box<Process> {
        let spinner = ProgressBar::new_spinner();
        let style = ProgressStyle::default_spinner()
            .tick_strings(
                &[
                    cformat!("<y>◶ </>").as_str(),
                    cformat!("<y>◵ </>").as_str(),
                    cformat!("<y>◷ </>").as_str(),
                    cformat!("<y>◴ </>").as_str(),
                ]
            )
            ;
        spinner.set_style(style);
        spinner.set_message(cformat!("<y>{}</>", message.into()));
        spinner.enable_steady_tick(Duration::from_millis(200));

        Box::from(Process { spinner })
    }

    fn finish_with_error<T: Into<String>>(&self, message: T) {
        self.spinner.finish_and_clear();
        cprintln!("<r>◉ {}</>", message.into());
    }

    fn finish_with_warning<T: Into<String>>(&self, message: T) {
        self.spinner.finish_and_clear();
        cprintln!("<y>◎ {}</>", message.into());
    }

    fn finish_with_success<T: Into<String>>(&self, message: T) {
        self.spinner.finish_and_clear();
        cprintln!("<g>✓ {}</>", message.into());
    }

    fn change_message<T: Into<String>>(&self, message: T) {
        self.spinner.set_message(cformat!("<y>{}</>", message.into()));
    }
}
