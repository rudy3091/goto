use super::*;

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
                    println!("cd {}", result[0]);
                } else {
                    // select
                    println!("{}", message::warn("More than one result found"));
                }
            } else {
                println!("{}", message::error("no bookmark file is found"));
            }
        }
    }
}
