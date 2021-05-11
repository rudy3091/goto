use super::*;
use super::super::message;

use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};


pub fn write<'a>(path_to_add: &'a String) -> Result<(), Box<dyn Error>> {
    let data_path = data_path().unwrap();
    let file_path = Path::new(CONSTANTS.datafile_name);
    let file_path = Path::new(&data_path).join(file_path);

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path);

    let mut file = match file {
        Ok(file) => file,
        Err(_) => {
            println!("{}", message::error("no bookmark file is found"));
            touch()?;
            OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap()
        }
    };

    let path_to_add = PathBuf::from(path_to_add);
    if let Ok(path) = fs::canonicalize(path_to_add) {
        file.write_all(&mut format!("{}\n", path.display()).as_bytes())?;
        println!("{}", message::success(format!("{} has been added to list!", path.display()).as_str()));
    } else {
        println!("{}", message::error("invalid path format"));
    }

    Ok(())
}

