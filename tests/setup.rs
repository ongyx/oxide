use std::path::PathBuf;
use std::{fs, io};

pub fn code() -> io::Result<Vec<PathBuf>> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests/code");

    Ok(fs::read_dir(root)?
        .filter_map(|entry| {
            let filename = entry.ok()?.path();
            if filename.extension()? == "oxide" {
                Some(filename)
            } else {
                None
            }
        })
        .collect())
}
