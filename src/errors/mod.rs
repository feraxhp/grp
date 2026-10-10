pub mod usettings;
pub mod fs_errors;
pub mod inquire;
pub mod general;
pub mod cripto;


pub trait ToError {
    type Context<'a>;
    
    fn to_error<'a>(&self, context: Self::Context<'a>) -> grp_core::Error;
}
