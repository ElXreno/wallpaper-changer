use std::path::PathBuf;
use std::error::Error;

pub fn create_dir(path: &PathBuf) {
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(()) => println!("{} dir created successfully!", &path.display()),
            Err(e) => panic!("Error {}", e.description()),
        }
    } else if !path.is_dir() {
        panic!(
            "{} already exists and is not a directory, exiting.",
            &path.display()
        );
    }
}