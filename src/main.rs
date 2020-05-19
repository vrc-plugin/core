const VRC_LOG_LOCATION:  &str = "\\AppData\\LocalLow\\VRChat\\VRChat";

fn main() {
    match vrc_log_path() {
        None => println!("Log file not found"),
        Some(s) => println!("log file {}", &s),
    }
}

fn vrc_log_path() -> Option<String> {
     if let Some(v) = dirs::home_dir() {
         if let Some(v) = v.to_str() {
             return Some(v.to_string() +  VRC_LOG_LOCATION);
         }
     }

     None
}