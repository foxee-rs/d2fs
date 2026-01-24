use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    sync::OnceLock,
};
use tokio::fs::create_dir_all;

static ROOT_FS: OnceLock<PathBuf> = OnceLock::new();
pub async fn init_fsroot() -> Result<PathBuf> {
    let data_dir = dirs::data_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "can't determine data directory"))?;
    let rootfs = data_dir.join("d2fs").join("rootfs");
    create_dir_all(&rootfs).await?;
    Ok(ROOT_FS.get_or_init(|| rootfs).clone())
}

pub fn fsroot() -> PathBuf {
    ROOT_FS.get().expect("rootfs must be initialized before use").clone()
}
