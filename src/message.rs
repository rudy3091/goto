pub fn error(message: &str) -> String {
    format!("\x1b[41m\x1b[30m ERROR \x1b[0m \x1b[31m{}\x1b[0m", message)
}

pub fn warn(message: &str) -> String {
    format!(
        "\x1b[43m\x1b[30m WARNING \x1b[0m \x1b[33m{}\x1b[0m",
        message
    )
}

pub fn success(message: &str) -> String {
    format!(
        "\x1b[42m\x1b[30m SUCCESS \x1b[0m \x1b[32m{}\x1b[0m",
        message
    )
}
