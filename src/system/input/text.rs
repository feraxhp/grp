use color_print::cformat;
use grp_core::Error;
use inquire::CustomUserError;
use inquire::Text;
use inquire::validator::Validation;
use regex::Regex;

use crate::system::input::Input;
use crate::errors::ToError;


impl Input {
    pub fn identifier<M, S>(message : M, description: Option<S>) -> Result<String, Error> 
    where 
        M: AsRef<str>,
        S: AsRef<str>,
    {
        let mut promp = Text::new(message.as_ref())
            .with_validator(identifier_validator)
        ;
        
        if let Some(desc) = description.as_ref().map(|s| s.as_ref()) {
            promp = promp.with_help_message(desc);
        }
        
        promp.prompt().map_err(|e| e.to_error(()))
    }
}

fn identifier_validator(input: &str) -> Result<Validation, CustomUserError> {
    let re = Regex::new(r"[^a-zA-Z0-9_]").unwrap();

    let positions: Vec<usize> = re.find_iter(input)
        .map(|mat| mat.start())
        .collect();
    
    match positions.len() {
        0 => Ok(Validation::Valid),
        _ => {
            // 11 is the length of the string after formating <r>c</> with color-print
            let reconstructed = input
                .chars()
                .enumerate()
                .fold(
                    String::with_capacity(input.len() + (positions.len() * 11) + 19), 
                    |mut string, (index, ch)| {
                        if index == 0 { string.push_str(&cformat!("<c>invalid: </>").as_str()); } // width: 19
                        
                        match (positions.contains(&index), ch) {
                            (true, ' ') => string.push_str(cformat!("<R> </>").as_str()),
                            (true, ch) => string.push_str(cformat!("<r>{ch}</>").as_str()),
                            (false, ch) => string.push(ch),
                        };
                        
                        string
                    }
                );
            
            Ok(Validation::Invalid(reconstructed.into()))
        }
    }
}
