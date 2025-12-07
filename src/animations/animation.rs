use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub(crate) struct Process { pub(crate) multi: MultiProgress, pub spinners: Vec<ProgressBar> }
pub(crate) struct Create  { pub spinner: ProgressBar }
pub(crate) struct Delete  { pub spinner: ProgressBar }
pub(crate) struct Fetch   { pub spinner: ProgressBar }

pub(crate) trait Style {
    fn normal() -> (ProgressStyle, u64);
    fn progress(template: &str) -> ProgressStyle {
        ProgressStyle::with_template(template)
        .unwrap()
        // .progress_chars("▰▱")
        .progress_chars("─●─")
    }
}

pub trait Subprogress {
    fn add(&mut self) -> usize;
    fn set_total(&self, index: usize, current: u64, template: &str);
    fn set_state(&self, index: usize, current: u64);
    fn set_message<T: Into<String>>(&self, index: usize, message: T);
    fn finish_all(&self);
}