use super::*;

pub fn run(cmd: &arg::Command) {
    println!("running command: {:?}", cmd);

    match cmd {
        arg::Command::Add(_cmd, _path) => {
            if let Ok(_) = data::write(&String::from(_path)) {
                println!("\x1b[33m{}\x1b[0m has been added!", _path);
            } else {
                println!("failed to write data");
            }
        }
        arg::Command::Go(target) => {
            if let Ok(contents) = data::read_file() {
                let result = query::search(target.as_str(), contents.as_str());

                if result.len() == 1 {
                    // go!
                    println!("cd {}", result[0]);
                } else {
                    // select
                    println!("so many results");
                }
            }
        }
    }
}
