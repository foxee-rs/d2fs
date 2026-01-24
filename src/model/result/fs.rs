use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FsKind {
    Dir,
    File,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsItem {
    pub kind: FsKind,
    pub name: String,
    pub size: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsLsResult {
    pub path: Vec<FsItem>,
}
