pub(crate) mod helpers;


#[allow(non_snake_case, non_upper_case_globals)]
pub mod ErrorType {
    pub const category: &str = "grp_core";
    macro_rules! base { ($string:literal) => { concat!("grp_core::", $string) }; }
    
    pub mod parsing {
        pub const category: &str = base!("parsing");
        macro_rules! parsing { ($string:literal) => { concat!(base!("parsing::"), $string) }; }
        
        pub const url:   &str = parsing!("url");
        pub const serde: &str = parsing!("serde");
        pub const issue: &str = parsing!("issue");
    }
    
    pub mod request {
        pub const category: &str = base!("request");
        macro_rules! request { ($string:literal) => { concat!(base!("request::"), $string) }; }
        
        pub const bad_token_scope: &str = request!("bad_token_scope");
        pub const unauthorized:    &str = request!("unauthorized");
        pub const getting_body:    &str = request!("get::body");
        pub const unsuported:      &str = request!("unsuported");
        pub const unknown:         &str = request!("unknown");
        pub const fetch:           &str = request!("fetch");
    }
    
    pub mod unsuported {
        pub const category: &str = base!("unsuported");
        macro_rules! unsuported { ($string:literal) => { concat!(base!("unsuported::"), $string) }; }
        
        pub const variant: &str = unsuported!("variant");
        pub const action:  &str = unsuported!("action");
    }
    
    pub mod not_found {
        pub const category: &str = base!("not_found");
        macro_rules! not_found { ($string:literal) => { concat!(base!("not_found::"), $string) }; }
        
        pub const organization: &str = not_found!("organization");
        pub const repository:   &str = not_found!("repository");
        pub const resource:     &str = not_found!("resource");
        pub const issue:        &str = not_found!("issue");
        pub const user:         &str = not_found!("user");
    }
    
    pub mod existing {
        pub const category: &str = base!("existing");
        macro_rules! existing { ($string:literal) => { concat!(base!("existing::"), $string) }; }
        
        pub const organization: &str = existing!("organization");
        pub const repository:   &str = existing!("repository");
        // pub const resource:     &str = existing!("resource");
        // pub const issue:        &str = existing!("issue");
        // pub const user:         &str = existing!("user");
    }
}
