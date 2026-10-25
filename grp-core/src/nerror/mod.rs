
#[allow(non_snake_case, non_upper_case_globals)]
pub mod ErrorType {
    macro_rules! base { ($string:literal) => { concat!("grp_core::", $string) }; }
    
    pub mod parsing {
        macro_rules! parsing { ($string:literal) => { concat!(base!("parsing::"), $string) }; }
        
        pub const url:   &str = parsing!("url");
        pub const serde: &str = parsing!("serde");
        pub const issue: &str = parsing!("issue");
    }
    
    pub mod request {
        macro_rules! request { ($string:literal) => { concat!(base!("request::"), $string) }; }
        
        pub const bad_token_scope: &str = request!("bad_token_scope");
        pub const unauthorized:    &str = request!("unauthorized");
        pub const getting_body:    &str = request!("getting_body");
        pub const unsuported:      &str = request!("unsuported");
        pub const fetch:           &str = request!("fetch");
    }
    
    pub mod unsuported {
        macro_rules! unsuported { ($string:literal) => { concat!(base!("unsuported::"), $string) }; }
        
        pub const action: &str = unsuported!("action");
    }
    
    pub mod not_found {
        macro_rules! not_found { ($string:literal) => { concat!(base!("not_found::"), $string) }; }
        
        pub const organization: &str = not_found!("organization");
        pub const repository:   &str = not_found!("repository");
        pub const user:         &str = not_found!("user");
    }
    
    pub mod existing {
        macro_rules! existing { ($string:literal) => { concat!(base!("existing::"), $string) }; }
        
        pub const organization: &str = existing!("organization");
        pub const repository:   &str = existing!("repository");
        pub const user:         &str = existing!("user");
    }
}
