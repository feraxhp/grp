use std::fmt::Display;

use color_print::cformat;
use grp_core::{Error, Formater, empty_notes};

macro_rules! etype {
    ($literal:literal) => { concat!("cripto::", $literal) };
}

pub struct CriptoError;


impl CriptoError {
    pub fn incomplete_payload(length: usize, expected: usize) -> Error {
        let etype = etype!("incomplete_payload");
        
        Error::new(
            etype, 
            cformat!("Try to decript an <i>incomplete payload</>"), 
            format!("The given payload length is {length} < {expected}"),
            vec![
                cformat!("Please check your config file").as_tip()
            ], 
            empty_notes!()
        )
    }
    
    pub fn nonce_error<D: Display>(e: D) -> Error {
        let etype = etype!("nonce");
        
        Error::new(
            etype, 
            cformat!("Error while trying to get the <i>nonce</>"), 
            format!("{e}"),
            vec![
                cformat!("Please check your config file").as_tip()
            ], 
            empty_notes!()
        )
    }
    
    pub fn dencrypt() -> Error {
        let etype = etype!("dencrypt");
        
        Error::new(
            etype, 
            cformat!("Invalid password or corrupted data"), 
            format!("The payload can't be decripted"),
            vec![
                cformat!("Please check your config file").as_tip()
            ], 
            empty_notes!()
        )
    }

    pub fn encrypt<D: Display>(e: D) -> Error {
        let etype = etype!("encrypt");
        
        Error::new(
            etype, 
            format!("The encription did not succeed"), 
            format!("{e}"),
            vec![], 
            empty_notes!()
        )
    }

    pub fn from_argon2(error: argon2::Error) -> Error {
        macro_rules! etype_argon2 {
            ($literal:literal) => { concat!(etype!("argon2::error"), $literal) };
        }
        
        match error {
            argon2::Error::PwdTooLong => {
                Error::new(
                    etype_argon2!("pwd_too_long"), 
                    format!("The provided password is to long"), 
                    format!("Consider adding a shorter one"),
                    vec![], 
                    empty_notes!()
                )
            },
            argon2::Error::SaltTooShort => {
                Error::new(
                    etype_argon2!("salt_too_short"), 
                    format!("This error must be unrechable"), 
                    format!("The salt is too short to be used as salt"),
                    vec![
                        cformat!("Please report these error: <y>{error}</>").as_tip()
                    ], 
                    empty_notes!()
                )
            },
            argon2::Error::SaltTooLong => {
                Error::new(
                    etype_argon2!("salt_too_long"), 
                    format!("This error must be unrechable"), 
                    format!("The salt is too long to be used as salt"),
                    vec![
                        cformat!("Please report these error: <y>{error}</>").as_tip()
                    ], 
                    empty_notes!()
                )
            },
            argon2::Error::OutputTooShort => {
                Error::new(
                    etype_argon2!("output_too_short"),
                    format!("This error must be unreachable"),
                    format!("The requested output buffer is too short"),
                    vec![
                        cformat!("Please report this error: <y>{error}</>").as_tip()
                    ],
                    empty_notes!()
                )
            },
            argon2::Error::OutputTooLong => {
                Error::new(
                    etype_argon2!("output_too_long"),
                    format!("This error must be unreachable"),
                    format!("The requested output buffer is too long"),
                    vec![
                        cformat!("Please report this error: <y>{error}</>").as_tip()
                    ],
                    empty_notes!()
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
                Error::new(
                    etype_argon2!("unrechable"), 
                    format!("This error must be unrechable"), 
                    format!("{error}"),
                    vec![
                        cformat!("Please report these error: <y>{error}</>").as_tip()
                    ], 
                    empty_notes!()
                )
            },
        }
    }
}
