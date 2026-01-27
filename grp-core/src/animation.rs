/// # Animation
/// This trait is meant to allow you to show progress during the platform interaction. 
pub trait Animation {
    fn new<T: Into<String>>(message: T) -> Box<Self>;
    fn finish_with_error<T: Into<String>>(&self, message: T);
    fn finish_with_warning<T: Into<String>>(&self, message: T);
    fn finish_with_success<T: Into<String>>(&self, message: T);
    fn change_message<T: Into<String>>(&self, message: T);
}

/// # animation::None
/// default implementation for a None animation. use it when its not necesary show 
/// any information to the user. 
pub struct None;

impl None { fn new() -> Box<None> { Box::new(None) } }

#[allow(unused_variables)]
impl Animation for None {
    fn new<T: Into<String>>(message: T) -> Box<Self> { Self::new() }
    fn finish_with_error<T: Into<String>>(&self, message: T) { }
    fn finish_with_warning<T: Into<String>>(&self, message: T) {  }
    fn finish_with_success<T: Into<String>>(&self, message: T) {  }
    fn change_message<T: Into<String>>(&self, message: T) {  }
}