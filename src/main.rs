// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod local;
mod system;
mod update;
mod commands;
mod animations;

use crate::commands::grp;

#[tokio::main]
async fn main() {
    // Completitions managger
    clap_complete::CompleteEnv::with_factory(grp::command).complete();
    
    // Read actual command
    grp::mannager(&grp::command().get_matches()).await;
}
