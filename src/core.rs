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
                let idx = buf.trim().parse::<usize>().unwrap();
                // println!("{}", idx);
                println!("cd {}", result[idx]);
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
                    for (idx, item) in result.iter().enumerate() {
                        println!("{}: {}", idx, item);
                    }
                    // // select
                    // println!("{}", message::warn("More than one result found"));
                    // let mut list = String::new();
                    // for item in result {
                    //     list.push_str(format!(";{}\n", item).as_str());
                    // }
                    // println!("{}", list);
                    process::exit(2);
                    // // let mut input = String::new();
                    // // stdin().read_line(&mut input).expect("failed to read");
                }
            } else {
                println!("{}", message::error("no bookmark file is found"));
            }
        }
    }
}
