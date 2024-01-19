use std::fs;
use std::error::Error;

pub fn read_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(file_path).map_err(|e| e.into())
}