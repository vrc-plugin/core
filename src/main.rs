extern crate glob;

const VRC_LOG_LOCATION: &str = "\\AppData\\LocalLow\\VRChat\\VRChat\\";

use std::fs::{read_dir, ReadDir, metadata, DirEntry};
use std::path::PathBuf;
use std::{fs, io};

fn main() {
    let mut path = vrc_log_path().expect("user directory not detected");
    println!("log file {}", &path);
    let log = get_latest_log(vrc_log_list(path).expect(""));
    println!("log file {:?}", log)
}

fn vrc_log_path() -> Option<String> {
    if let Some(v) = dirs::home_dir() {
        if let Some(v) = v.to_str() {
            return Some(v.to_string() + VRC_LOG_LOCATION);
        }
    }

    None
}

fn vrc_log_list<'a>(dir_path: String) ->  Result<Vec<PathBuf>, io::Error>  {
    Ok(fs::read_dir(dir_path)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r:&PathBuf| r.file_name().unwrap().to_str().unwrap().contains("output"))
        .collect())
}

fn get_latest_log<'a>(mut logs : Vec<PathBuf>) -> String {
    logs.sort_by_key(|p| p.metadata().expect("").created().expect(""));
    logs.reverse();
    logs.get(0).expect("").to_str().unwrap().to_string()
}
