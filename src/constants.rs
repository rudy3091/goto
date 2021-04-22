pub struct Constants {
    pub datafile_name: &'static str,
    pub shell_rcfile_name: &'static str,
}

pub static CONSTANTS: &'static Constants = &Constants {
    datafile_name: "data.txt",
    shell_rcfile_name: "rc.sh",
};
