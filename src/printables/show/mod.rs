pub mod comment;
pub mod pconf;
pub mod issue;
pub mod repo;
pub mod user;

use std::sync::LazyLock;
use color_print::cformat;
use grp_core::{Config, Platform};
use regex::Regex;

use super::FormatTools;

pub trait Show {
    type Context;
    
    fn print_pretty(&self, context: Self::Context) {
        self.to_string_iter(context).for_each(|s| {
            println!("{}", s)
        });
    }
    fn to_string_iter(&self, context: Self::Context) -> impl Iterator<Item = String> + '_;
}

impl Show for Vec<String> {
    type Context = ();
    
    fn to_string_iter(&self, _: Self::Context) -> impl Iterator<Item = String> + '_ {
        self.iter().map(|s| s.to_owned())
    }
}

pub trait Empty {
    fn empty(&self) -> String;
}

impl Empty for String {
    fn empty(&self) -> String {
        match self.len() {
            0 => cformat!("<dim,i><<empty>></dim,i>"),
            _ => self.to_string(),
        }
    }
}

impl Empty for &String {
    fn empty(&self) -> String {
        match self.len() {
            0 => cformat!("<dim,i><<empty>></dim,i>"),
            _ => self.to_string(),
        }
    }
}

pub(crate) fn replace_user_mentions(
    text: &str,
    platform: &Platform,
    config: &Config,
) -> String {
    static MENTION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\B@([a-zA-Z0-9_](?:[a-zA-Z0-9_\-]*[a-zA-Z0-9_])?)").unwrap()
    });

    MENTION_REGEX.replace_all(text, |caps: &regex::Captures| {
        let name = &caps[1];
        let url = platform.get_user_url(config, name);
        format!("@{name}").as_link(&url)
    }).into_owned()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_user_mentions_func() {
        let config = Config::new("gh", "feraxhp", "token", "api.github.com");
        let platform = Platform::Github;

        let input = "Hello @feraxhp, contact test@example.com or @feraxhp-bot (@feraxhp)";
        let output = replace_user_mentions(input, &platform, &config);

        let expected_user_link = "@feraxhp".as_link("https://github.com/feraxhp");
        let expected_bot_link = "@feraxhp-bot".as_link("https://github.com/feraxhp-bot");

        assert!(output.contains(&expected_user_link));
        assert!(output.contains(&expected_bot_link));
        assert!(output.contains("test@example.com"));
        assert!(!output.contains("mailto:"));
    }
}
