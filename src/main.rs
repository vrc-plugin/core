extern crate glob;

const VRC_LOG_LOCATION: &str = "\\AppData\\LocalLow\\VRChat\\VRChat\\";

use self::glob::glob;
use std::fs::{read_dir, ReadDir, metadata, DirEntry};
use std::{fs, io};
use std::path::PathBuf;

fn main() {
    let mut path = vrc_log_path().expect("user directory not detected");
    println!("log file {}", &path);
    for x in vrc_latest_log(path) {
        for y in x {
            println!("{:?}", y)
        }
    }
}

fn vrc_log_path() -> Option<String> {
    if let Some(v) = dirs::home_dir() {
        if let Some(v) = v.to_str() {
            return Some(v.to_string() + VRC_LOG_LOCATION);
        }
    }

    None
}

fn vrc_latest_log(dir_path: String) -> Option<&PathBuf> {
    let mut res : Vec<PathBuf> = Ok(fs::read_dir(dir_path)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.file_name().unwrap().to_str().unwrap().contains("output")) // Filter out non-folders
        .collect()).expect("");
    res.sort_by_key(|p : &PathBuf| p.metadata().unwrap().created().unwrap());

    return res.get(0)


}