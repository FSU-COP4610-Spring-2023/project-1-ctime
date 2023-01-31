pub mod IORedirection {
    use std::fs;
    use std::io::Write;
    use std::fs::OpenOptions;

    pub fn overwrite(data : &str, outfile : &str) {
        std::fs::File::open(outfile).ok();
        std::fs::write(outfile, data).expect("Unable to write file");
        println!("Redirected");
    }

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