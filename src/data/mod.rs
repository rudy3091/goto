pub mod create;

use super::constants::CONSTANTS;

use std::error::Error;
use std::{env, fs, io, io::prelude::*};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::ffi::OsString;

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
    let file_path = Path::new(CONSTANTS.datafile_name);
    let file_path = Path::new(&data_path).join(file_path);

    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn touch() -> Result<(), io::Error> {
    let data_path = data_path().unwrap();
    let file_path = Path::new(CONSTANTS.datafile_name);
    let file_path = Path::new(&data_path).join(file_path);
    println!("created data file at {}", file_path.display());
    match OpenOptions::new().create(true).write(true).open(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn get_shell_script_path() -> OsString {
    let mut sc_path = data_path().unwrap();
    sc_path.pop();
    sc_path.pop();

    Path::join(&sc_path, CONSTANTS.shell_rcfile_name).into_os_string()
}
