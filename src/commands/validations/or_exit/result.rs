use std::process::exit;

use mynox::Error;

use super::structure::OrExit;


impl<T> OrExit for Result<T, Error> {
    type Output =  T;
    
    fn or_exit<A: grp_core::animation::Animation + ?Sized>(self, animation: &Box<A>) -> Self::Output {
        match self {
            Ok(p) => p,
            Err(e) => {
                animation.finish_with_error(e.message());
                e.eprint();
                exit(1)
            },
        }
    }
}
