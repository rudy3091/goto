use super::constants::CONSTANTS;
use super::message;

use std::error::Error;
use std::{env, fs, io, io::prelude::*};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

pub fn sym_link_path(path: &PathBuf) -> io::Result<PathBuf> {
    let current_dir = fs::read_link(path)?;
    Ok(current_dir)
}

pub fn data_path() -> io::Result<PathBuf> {
    let current_dir = env::current_exe()?;
    let mut current_dir = sym_link_path(&current_dir).unwrap();
    current_dir.pop();

    // current_dir -> binary file directory (go-to/target/debug/..)
    Ok(current_dir)
}

pub fn read_file() -> Result<String, Box<dyn Error>> {
    let data_path = data_path().unwrap();
    let file_path = Path::new(CONSTANTS.datafile_path_suffix);
    let file_path = Path::new(&data_path).join(file_path);

    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn touch() -> Result<(), io::Error> {
    let data_path = data_path().unwrap();
    let file_path = Path::new(CONSTANTS.datafile_path_suffix);
    let file_path = Path::new(&data_path).join(file_path);
    println!("created data file at {}", file_path.display());
    match OpenOptions::new().create(true).write(true).open(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn write<'a>(path_to_add: &'a String) -> Result<(), Box<dyn Error>> {
    let data_path = data_path().unwrap();
    let file_path = Path::new(CONSTANTS.datafile_path_suffix);
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
