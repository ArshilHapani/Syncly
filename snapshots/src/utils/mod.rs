use std::{
    fs,
    io::{Error, Write},
    path::PathBuf,
};

pub fn modify_file_content(path: &PathBuf, content: &[u8], truncate: bool) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(truncate)
        .open(path)?;

    file.write_all(content)?;
    file.sync_all()?;
    file.flush()?;

    Ok(())
}
