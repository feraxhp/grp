use std::time::Duration;
use color_print::{cformat, ceprintln};
use indicatif::{ProgressBar, ProgressStyle};
use crate::animations::animation::{Delete, Suspend};
use grp_core::animation::Animation;



impl Animation for Delete {
    fn new<T: Into<String>>(message: T) -> Box<Delete> {
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
        spinner.set_message(cformat!("<y>{}</>", message.into()));
        spinner.enable_steady_tick(Duration::from_millis(200));

        Box::from(Delete { spinner })
    }

    fn finish_with_error<T: Into<String>>(&self, message: T) {
        self.spinner.finish_and_clear();
        ceprintln!("<r>(--⚡--) {}</>", message.into());
    }

    fn finish_with_warning<T: Into<String>>(&self, message: T) {
        self.spinner.finish_and_clear();
        ceprintln!("<y>💻--!--🌎 {}</>", message.into());
    }

    fn finish_with_success<T: Into<String>>(&self, message: T) {
        self.spinner.finish_and_clear();
        ceprintln!("<g>(--✻--) {}</>", message.into());
    }

    fn change_message<T: Into<String>>(&self, message: T) {
        self.spinner.set_message(cformat!("<y>{}</>", message.into()));
    }
}



impl Suspend for Delete {
    fn suspend<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R 
    { self.spinner.suspend(f) }
}
