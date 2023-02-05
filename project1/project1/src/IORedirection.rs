pub mod IORedirection {
    
    use std::io::Write;
    use std::fs::OpenOptions;

//    use std::os::fd::{AsRawFd};
    use std::os::unix::io::AsRawFd;

    pub fn overwrite(outfile : &str) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(outfile)
            .expect("Unable to open file");
            
        std::fs::write(outfile, "").expect("Unable to write file");
        nix::unistd::dup2(file.as_raw_fd(), std::io::stdout().as_raw_fd()).ok();
    }

    //Everything below this needs to be updated. Only overwrite() is working correctly
    pub fn append(outfile : &str) {
        let appendFile = OpenOptions::new()
            .append(true)
            .create(true)
            .open(outfile)
            .expect("Unable to open file");

        nix::unistd::dup2(appendFile.as_raw_fd(), std::io::stdout().as_raw_fd()).ok();
    }

    pub fn readFile(file : &str) {
        //let file = std::fs::File::open(infile);
        let infile = OpenOptions::new()
            .read(true)
            .open(file)
            .expect("Unable to open file");

        nix::unistd::dup2(infile.as_raw_fd(), std::io::stdin().as_raw_fd()).ok();
    }
}
