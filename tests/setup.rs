use std::path::PathBuf;
use std::{fs, io};

pub fn examples() -> io::Result<Vec<PathBuf>> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("examples");

    Ok(fs::read_dir(root)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let filename = entry.path();
            if filename.extension()? == "oxide" {
                Some(filename)
            } else {
                None
            }
        })
        .collect())
}
