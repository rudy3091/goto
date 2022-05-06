use std::env;

#[derive(PartialEq, Debug)]
pub enum Command {
    Init, // loads shell scripts
    Add(String, String), // add path to a data file
    Prompt(String), // gets input from stdin
    Go(String), // execute chdir
}

impl Command {
    pub fn new(args: &[String]) -> Result<Command, &str> {
        if args.len() < 2 {
            return Err("invalid command");
        }

        let cmd = match read_command(args).as_str() {
            "init" => Command::Init,
            "add" => Command::Add(String::from(&args[1]), String::from(&args[2])),
            "prompt" => Command::Prompt(String::from(&args[2])),
            _ => Command::Go(String::from(&args[1])),
        };

        Ok(cmd)
    }
}

pub fn read() -> Vec<String> {
    env::args().collect()
}

pub fn read_command(args: &[String]) -> String {
    args[1].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_init_command() {
        let bin = String::from("bin");
        let cmd = String::from("init");
        let cmd = Command::new(&[bin, cmd]).unwrap();
        assert_eq!(cmd, Command::Init);
    }

    #[test]
    fn gets_add_command() {
        let bin = String::from("bin");
        let cmd = String::from("add");
        let path = String::from(".");
        let cmd = Command::new(&[bin, cmd, path]).unwrap();
        assert_eq!(cmd, Command::Add(String::from("add"), String::from(".")));
    }

    #[test]
    fn gets_prompt_command() {
        let bin = String::from("bin");
        let cmd = String::from("prompt");
        let path = String::from(".");
        let cmd = Command::new(&[bin, cmd, path]).unwrap();
        assert_eq!(cmd, Command::Prompt(String::from(".")));
    }

    #[test]
    fn gets_go_command() {
        let bin = String::from("bin");
        let path = String::from("path");
        let cmd = Command::new(&[bin, path]).unwrap();
        assert_eq!(cmd, Command::Go(String::from("path")));
    }
}
