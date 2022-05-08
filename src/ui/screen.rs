pub fn load() {
    print!("\x1b[?1049h\x1b[H");
}

pub fn unload() {
    print!("\x1b[?1049l");
}
