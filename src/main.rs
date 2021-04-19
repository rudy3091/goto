mod arg;
mod constants;
mod core;
mod data;
mod message;
mod query;

fn main() {
    let args = arg::read();
    let cmd = arg::Command::new(&args).unwrap();

    core::run(&cmd);
}
