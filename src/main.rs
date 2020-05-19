const VRC_LOG_LOCATION:  &str = "\\AppData\\LocalLow\\VRChat\\VRChat";

fn main() {

    print!("{}", vrc_log_path());
}

fn vrc_log_path() -> String {
     if let Some(v) = dirs::home_dir() {
         if let Some(v) = v.to_str() {
             return v.to_string() +  VRC_LOG_LOCATION;
         }
     }

     "".to_string()
}