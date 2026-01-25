use crate::local::git::structs::GitUtils;
use git2::{Cred, RemoteCallbacks};
use grp_core::config::Config;


impl GitUtils {
    pub(crate) fn get_credential_callbacks(conf: &Config) -> RemoteCallbacks<'static>{
        let mut callbacks = RemoteCallbacks::new();
        let token = conf.token.clone();
        
        callbacks.credentials(move |_, _, _| {
            Cred::userpass_plaintext("oauth2", &token)
        });

        callbacks
    }
}