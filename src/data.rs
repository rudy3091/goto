use super::constants::CONSTANTS;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::{env, io, io::prelude::*};

pub fn bin_path() -> io::Result<PathBuf> {
    let mut current_dir = env::current_exe()?;
    current_dir.pop();
    current_dir.pop();
    current_dir.pop();

    // current_dir -> project root directory (src/..)
    Ok(current_dir)
}

pub fn read_file() -> Result<String, Box<dyn Error>> {
    let bin_path = bin_path().unwrap();
    let file_path = Path::new(CONSTANTS.datafile_path_suffix);
    let file_path = Path::new(&bin_path).join(file_path);

    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn write<'a>(data: &'a String) -> Result<(), Box<dyn Error>> {
    let bin_path = bin_path().unwrap();
    let file_path = Path::new(CONSTANTS.datafile_path_suffix);
    let file_path = Path::new(&bin_path).join(file_path);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    file.write_all(&mut format!("{}\n", data).as_bytes())?;

    Ok(())
}
