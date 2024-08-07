use std::path::PathBuf;

use crate::errors::{OmniverlayError, OmniverlayResult};

pub fn get_profiles_dir() -> OmniverlayResult<PathBuf> {
    let omniverlay_dir = get_omniverlay_dir()?;
    Ok(omniverlay_dir.join("profiles"))
}

pub fn get_home_dir() -> OmniverlayResult<PathBuf> {
    let home_dir = dirs::home_dir();
    if let Some(home_dir) = home_dir {
        Ok(home_dir)
    } else {
        Err(OmniverlayError::AppDirectory)
    }
}

pub fn get_omniverlay_dir() -> OmniverlayResult<PathBuf> {
    let home_dir = get_home_dir()?;

    Ok(home_dir.join(".omniverlay"))
}