use std::{
    fs,
    io::{Error, Write},
    path::Path,
};

pub fn modify_file_content(path: &Path, content: &[u8], truncate: bool) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(truncate)
        .open(path)?;

    file.write_all(content)?;
    file.sync_all()?;
    file.flush()?;

    Ok(())
}
