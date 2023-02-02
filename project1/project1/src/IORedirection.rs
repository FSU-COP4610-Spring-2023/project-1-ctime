pub mod IORedirection {
    use std::fs;
    use std::io::Write;
    use std::fs::OpenOptions;

    use std::os::fd::{AsRawFd, AsFd};
    use nix::errno::Errno;

    pub fn overwrite(outfile : &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(outfile)
            .expect("Unable to open file");
        std::fs::write(outfile, "").expect("Unable to write file");
        nix::unistd::dup2(file.as_raw_fd(), std::io::stdout().as_raw_fd()).ok();
    }

    //Everything below this needs to be updated. Only overwrite() is working correctly
    pub fn append(data : &str, outfile : &str) {
        let mut appendFile = OpenOptions::new()
            .append(true)
            .create(true)
            .open(outfile)
            .expect("Unable to open file");

        appendFile.write_all("\n".as_bytes()).expect("Unable to write file");
        appendFile.write_all(data.as_bytes()).expect("Unable to write file");
        println!("Appended");
    }

    pub fn readFile(infile : &str) -> String {
        return fs::read_to_string(infile).expect("Unable to read file");
    }
}