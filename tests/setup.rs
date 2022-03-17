use std::path::PathBuf;
use std::{fs, io};

static ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

pub fn relpath(path: &str) -> PathBuf {
    [ROOT, path].iter().collect()
}

pub fn find_ext(path: PathBuf, ext: &str) -> io::Result<Vec<PathBuf>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let filename = entry.ok()?.path();
            if filename.extension()? == ext {
                Some(filename)
            } else {
                None
            }
        })
        .collect())
}
