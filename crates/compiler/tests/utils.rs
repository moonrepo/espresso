use std::fs;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path.as_ref()).unwrap()
}
