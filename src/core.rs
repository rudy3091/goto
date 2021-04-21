use super::*;
use std::process;
// use std::io::stdin;

pub fn run(cmd: &arg::Command) {
    match cmd {
        arg::Command::Add(_cmd, _path) => {
            if let Err(_) = data::write(&String::from(_path)) {
                println!("{}", message::error("failed to write data"));
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
                } else if result.len() == 0 {
                    println!("{}", message::error("No matching result found"));
                    process::exit(1);
                } else {
                    // select
                    println!("{}", message::warn("More than one result found"));
                    let mut list = String::new();
                    for item in result {
                        list.push_str(format!("{}\n", item).as_str());
                    }
                    println!("{}", list);
                    process::exit(2);
                    // let mut input = String::new();
                    // stdin().read_line(&mut input).expect("failed to read");
                }
            } else {
                println!("{}", message::error("no bookmark file is found"));
            }
        }
    }
}
