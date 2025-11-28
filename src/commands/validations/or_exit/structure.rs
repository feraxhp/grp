
use crate::girep::animation::Animation;


pub trait OrExit {
    type Output;
    fn or_exit<A: Animation + ?Sized>(&self, animation: &Box<A>) -> Self::Output;
}
