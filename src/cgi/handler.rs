use std::process::{Child, Command, Stdio};
use std::io::Error;
pub struct CGIHandler {
    script_path: String,
}
impl CGIHandler {
    pub fn new(script_path: String) -> Self {
        CGIHandler { script_path }
    }
    pub fn handle_request(&self) -> Option<String> {
        return match Self::execute_script(&self.script_path) {
            Ok(child) => {
                if let Ok(output) = child.wait_with_output() {
                    return Some(String::from_utf8_lossy(&output.stdout).to_string());
                } else {
                    None
                }
            }
            Err(_) => None,
        };
    }
    fn execute_script(file_name: &str) -> Result<Child, Error> {
       Command::new(file_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("PATH_INFO", file_name)
            .spawn()
    }
}
