use std::process::exit;
use color_print::cformat;

use super::structure::OrExit;

use crate::girep::usettings::structs::Pconf;
use crate::girep::animation::Animation;


impl OrExit for Option<Pconf> {
    type Output = Pconf;
    
    fn or_exit<A: Animation + ?Sized>(&self, animation: &Box<A>) -> Self::Output {
        match self {
            Some(pconf) => pconf.clone(),
            None => {
                animation.finish_with_error(cformat!("No default <i,m>pconf</i,m> <r>configured</>"));
                exit(1)
            },
        }
    } 
}
