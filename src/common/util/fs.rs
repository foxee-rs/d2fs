use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    sync::OnceLock,
};
use tokio::fs::create_dir_all;

static ROOT_FS: OnceLock<PathBuf> = OnceLock::new();
pub async fn init_fsroot() -> Result<PathBuf> {
    let data_dir = dirs::data_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "can't determine data directory"))?;
    let fs_root = data_dir.join("d2fs").join("fsroot");
    create_dir_all(&fs_root).await?;
    let fs_document = fs_root.join("Document");
    create_dir_all(&fs_document).await?;
    let fs_download = fs_root.join("Download");
    create_dir_all(&fs_download).await?;
    Ok(ROOT_FS.get_or_init(|| fs_root).clone())
}

pub fn fsroot() -> PathBuf {
    ROOT_FS.get().expect("fsroot must be initialized before use").clone()
}
