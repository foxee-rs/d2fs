use std::{io, path::PathBuf};
use tokio::fs::create_dir_all;

pub async fn init_rootfs() -> io::Result<PathBuf> {
    let data_dir =
        dirs::data_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "can't to determine data directory"))?;
    let rootfs = data_dir.join("d2fs").join("rootfs");
    create_dir_all(&rootfs).await?;
    Ok(rootfs)
}
