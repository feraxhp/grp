pub(crate) trait Animation {
    fn new(message: &str) -> Box<Self>;
    fn finish_with_error(&self, message: &str);
    fn finish_with_warning(&self, message: &str);
    fn finish_with_success(&self, message: &str);
    fn change_message(&self, message: String);
}