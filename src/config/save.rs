// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::config::structure::Root;

pub(crate) fn save_config(config: &Root) -> Result<(), Box<dyn std::error::Error>> {
    let file_location = crate::config::location::get_location();
    let file = std::fs::File::create(file_location)?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}