use std::time::Duration;
use color_print::{cformat, cprintln};
use indicatif::{ProgressBar, ProgressStyle};

pub(crate) struct Process {
    spinner: ProgressBar,
}

impl Process {
    pub fn new(message: &str) -> Self {
        let spinner = ProgressBar::new_spinner();
        let style = ProgressStyle::default_spinner()
            .tick_strings(
                &[
                    cformat!("<y>⬪</>").as_str(),
                    cformat!("<y>⬨</>").as_str(),
                    cformat!("<y>⬦</>").as_str(),
                    cformat!("<y>⬥</>").as_str(),
                    cformat!("<y>⬧</>").as_str(),
                    cformat!("<y>⬪</>").as_str(),
                ]
            )
            ;
        spinner.set_style(style);
        spinner.set_message(cformat!("<y>{}</>", message.to_string()).to_string());
        spinner.enable_steady_tick(Duration::from_millis(200));

        Process { spinner }
    }

    pub fn finish_with_error(&self, message: &str) {
        self.spinner.finish_and_clear();
        cprintln!("<r>X {}</>", message.to_string());
    }

    pub fn finish_with_success(&self, message: &str) {
        self.spinner.finish_and_clear();
        cprintln!("<g>✓ {}</>", message.to_string());
    }
}
