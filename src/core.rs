use super::*;
use std::{process, io::BufRead};
// use std::io::stdin;

pub fn run(cmd: &arg::Command) {
    match cmd {
        arg::Command::Init => {
            if let Ok(p) = data::get_shell_script_path().into_string() {
                println!("{}", p);
            } else {
                println!("{}", message::error("invalid shell script path"));
            }
            process::exit(1);
        }

        arg::Command::Add(_cmd, _path) => {
            if let Err(_) = data::create::write(&String::from(_path)) {
                println!("{}", message::error("failed to write data"));
            }
        }

        arg::Command::Prompt(target) => {
            let stdin = std::io::stdin();
            let mut buf = String::new();
            stdin.lock().read_line(&mut buf).unwrap();

            if let Ok(contents) = data::read_file() {
                let result = query::search(target.as_str(), contents.as_str());
                let idx = buf.trim().parse::<usize>();
                if let Ok(n) = idx {
                    if n >= result.len() {
                        println!("{}", message::error("input is out of range"));
                        process::exit(1);
                    }

                    println!("cd {}", result[n]);
                } else {
                    println!("{}", message::error("expected number"));
                    process::exit(1);
                }
            }
        }

        arg::Command::Go(target) => {
            if let Ok(contents) = data::read_file() {
                let result = query::search(target.as_str(), contents.as_str());

                if result.len() == 1 {
                    /*
                     * good to go!
                     */
                    println!("cd {}", &result[0]);
                    process::exit(0);
                } else if result.len() == 0 {
                    println!("{}", message::error("No matching result found"));
                    process::exit(1);
                } else {
                    ui::screen::load();
                    println!("{}", message::warn("More than one result found"));
                    for (idx, item) in result.iter().enumerate() {
                        println!("{}: {}", idx, item);
                    }
                    process::exit(2);
                }
            } else {
                println!("{}", message::error("no bookmark file is found"));
            }
        }

        arg::Command::Close => {
            ui::screen::unload();
        }
    }
}
