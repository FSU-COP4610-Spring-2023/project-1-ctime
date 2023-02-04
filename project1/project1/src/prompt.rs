//mod envVar;
//use crate::envVar::envVar::replace;

pub mod prompt {
    pub fn print() {
        let mut testStr: String = "".to_string();
        //let user : String = "$USER".to_string();
        testStr.push_str(crate::envVar::envVar::replace("$USER".to_string()).as_str());
        testStr.push('@');
        testStr.push_str(crate::envVar::envVar::replace("$MACHINE".to_string()).as_str());
        testStr.push_str(" : ");
        testStr.push_str(crate::envVar::envVar::replace("$PWD".to_string()).as_str());
        testStr.push_str(" >");
        print!("{testStr}");
    }
}