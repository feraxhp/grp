
pub(crate) trait Animation {
    fn new<T: Into<String>>(message: T) -> Box<Self>;
    fn finish_with_error<T: Into<String>>(&self, message: T);
    fn finish_with_warning<T: Into<String>>(&self, message: T);
    fn finish_with_success<T: Into<String>>(&self, message: T);
    fn change_message<T: Into<String>>(&self, message: T);
}

pub struct None;

#[allow(unused_variables)]
impl Animation for None {
    fn new<T: Into<String>>(message: T) -> Box<Self> { unreachable!() }
    fn finish_with_error<T: Into<String>>(&self, message: T) { unreachable!() }
    fn finish_with_warning<T: Into<String>>(&self, message: T) { unreachable!() }
    fn finish_with_success<T: Into<String>>(&self, message: T) { unreachable!() }
    fn change_message<T: Into<String>>(&self, message: T) { unreachable!() }
}