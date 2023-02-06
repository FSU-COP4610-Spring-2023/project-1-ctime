pub mod prompt {
    use crate::direc;
    use direc::direc::find_curr_direc;
    
    pub fn print() {
        let mut testStr: String = "".to_string();
        testStr.push_str(crate::envVar::envVar::replace("$USER".to_string()).as_str());
        testStr.push('@');
        testStr.push_str(crate::envVar::envVar::replace("$MACHINE".to_string()).as_str());
        testStr.push_str(":");
	    testStr.push_str(find_curr_direc().as_str());
        testStr.push_str(" >");
        print!("{testStr}");
    }
}
