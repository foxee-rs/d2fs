use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsRoot {
    pub path: PathBuf,
}

impl Deref for FsRoot {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
