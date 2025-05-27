use git2::{Cred, RemoteCallbacks};
use crate::girep::config::config::Config;
use crate::girep::local::git_utils::structure::GitUtils;

impl GitUtils{
    pub(crate) fn get_credential_callbacks(conf: Config) -> RemoteCallbacks<'static>{
        let mut callbacks = RemoteCallbacks::new();

        callbacks.credentials(move |_, _, _| {
            Cred::userpass_plaintext("oauth2", conf.token.as_str())
        });

        callbacks
    }
}