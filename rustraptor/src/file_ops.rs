use std::path::Path;
use std::fs;
use std::error::Error;

pub fn read_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
    //
    // Function first validates user input. If validation passes,
    // the function will read the contents in a String format.
    //
    if !is_valid_file_path(file_path) {
        return Err("Invalid file path".into());
    }

    fs::read_to_string(file_path).map_err(|e| e.into())
}

fn is_valid_file_path(file_path: &str) -> bool {
    //
    // Function checks if a given input item on the file_path is a file using `is_file()` function,
    // which returns a boolean. The function also makes sure that the given item or path
    // exists by using the `exists()` function, returning a boolean.
    //
    let path = Path::new(file_path);
    path.is_file() && path.exists()
}