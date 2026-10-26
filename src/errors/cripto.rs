use std::fmt::Display;

use mynox::Formater;
use mynox::Error;
use mynox::empty_iter;
use mynox::error;


macro_rules! etype {
    ($literal:literal) => { concat!("cripto::", $literal) };
}

pub struct CriptoError;


impl CriptoError {
    pub fn incomplete_payload(length: usize, expected: usize) -> Error {
        let etype = etype!("incomplete_payload");
        
        Error::new(
            etype, 
            format!("Try to decript an _incomplete payload_"), 
            format!("The given payload length is {length} < {expected}"),
            vec![
                format!("Please check your config file").as_tip()
            ], 
            empty_iter!()
        )
    }
    
    pub fn nonce_error<D: Display>(e: D) -> Error {
        let etype = etype!("nonce");
        
        error!(
            etype,
            "Error while trying to get the _nonce_",
            format!("{e}"),
            => format!("Please check your config file").as_tip()
        )
    }
    
    pub fn dencrypt() -> Error {
        let etype = etype!("dencrypt");
        
        Error::new(
            etype, 
            format!("Invalid password or corrupted data"), 
            format!("The payload can't be decripted"),
            empty_iter!(),
            empty_iter!(),
        )
    }

    pub fn encrypt<D: Display>(e: D) -> Error {
        let etype = etype!("encrypt");
        
        Error::new(
            etype, 
            format!("The encription did not succeed"), 
            format!("{e}"),
            empty_iter![], 
            empty_iter!()
        )
    }

    pub fn from_argon2(error: argon2::Error) -> Error {
        macro_rules! etype_argon2 {
            ($literal:literal) => { concat!(etype!("argon2::error::"), $literal) };
        }
        
        match error {
            argon2::Error::PwdTooLong => {
                Error::from(
                    error,
                    etype_argon2!("pwd_too_long"), 
                    format!("The provided password is to long"), 
                    format!("Consider adding a shorter one"),
                    empty_iter![], 
                    empty_iter!()
                )
            },
            argon2::Error::SaltTooShort => {
                Error::from(
                    error,
                    etype_argon2!("salt_too_short"), 
                    format!("This error must be unrechable"), 
                    format!("The salt is too short to be used as salt"),
                    empty_iter!(),
                    vec![
                        format!("Please report these error: ={error}=").as_tip()
                    ], 
                )
            },
            argon2::Error::SaltTooLong => {
                Error::from(
                    error,
                    etype_argon2!("salt_too_long"), 
                    format!("This error must be unrechable"), 
                    format!("The salt is too long to be used as salt"),
                    empty_iter!(),
                    vec![
                        format!("Please report these error: ={error}=").as_tip()
                    ], 
                )
            },
            argon2::Error::OutputTooShort => {
                Error::from(
                    error,
                    etype_argon2!("output_too_short"),
                    format!("This error must be unreachable"),
                    format!("The requested output buffer is too short"),
                    empty_iter!(),
                    vec![
                        format!("Please report this error: ={error}=").as_tip()
                    ],
                )
            },
            argon2::Error::OutputTooLong => {
                Error::from(
                    error,
                    etype_argon2!("output_too_long"),
                    format!("This error must be unreachable"),
                    format!("The requested output buffer is too long"),
                    empty_iter!(),
                    vec![
                        format!("Please report this error: ={error}=").as_tip()
                    ],
                )
            },
            argon2::Error::AdTooLong |
            argon2::Error::AlgorithmInvalid |
            argon2::Error::B64Encoding(_) |
            argon2::Error::KeyIdTooLong |
            argon2::Error::MemoryTooLittle |
            argon2::Error::MemoryTooMuch |
            argon2::Error::SecretTooLong |
            argon2::Error::ThreadsTooFew |
            argon2::Error::ThreadsTooMany |
            argon2::Error::TimeTooSmall |
            argon2::Error::VersionInvalid |
            argon2::Error::OutOfMemory => {
                Error::from(
                    error,
                    etype_argon2!("unrechable"), 
                    format!("This error must be unrechable"), 
                    format!("{error}"),
                    empty_iter!(),
                    vec![
                        format!("Please report these error: ={error}=").as_tip()
                    ], 
                )
            },
        }
    }
}
