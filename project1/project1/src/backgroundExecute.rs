pub mod backgroundExecute{
 use std::io::{self};

    use std::ffi::CString;
    

    use nix::unistd::execv;
    
    use nix::{unistd::{fork, ForkResult}};
    use libc::pid_t;
    use nix::unistd::Pid;
    
    

    pub fn fill_arg_vector() -> Vec<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let tokens = input.split_whitespace();

        let mut args = Vec::new();

        for token in tokens {
	    args.push(token.to_string());
	}     

	args
    }

    
    pub fn convert_to_cstring(input: Vec<String>) -> Vec<CString> {
	let mut cargs = Vec::new();
	for i in input {
	    let cstring = CString::new(i).unwrap();
	    cargs.push(cstring);
	}	

	cargs
    }
    //same as exuction, just doesnt wait for the child to execute, and returns its process id
    pub fn background_execute(input: Vec<String>, input2: Vec<String>, rd: i32 ) -> pid_t{	
	let mut cargs = Vec::new();
        for i in input {
            let cstring = CString::new(i).unwrap();
            cargs.push(cstring);
        }

	let mut cargs2 = Vec::new();
        for i in input2 {
            let cstring2 = CString::new(i).unwrap();
            cargs2.push(cstring2);
        }
	
	match unsafe{fork()} {
            Ok(ForkResult::Parent { child, ..}) => {
                //waitpid(child , None).unwrap();
                println!("[{}]", child);
                child.as_raw()
            }
            Ok(ForkResult::Child) => {
                if rd == 1 {    //1: Output
                    let outfile = cargs[cargs.len() - 1].to_str();
                    crate::IORedirection::IORedirection::overwrite(outfile.unwrap());
                    cargs.pop();    //remove redirect token and output file from command vector
                    cargs.pop();
                }
                else if rd == 2 {   //2: Input
                    let infile = cargs[cargs.len() - 1].to_str();
                    crate::IORedirection::IORedirection::readFile(infile.unwrap());
                    cargs.pop();
                    cargs.pop();
                }
                else if rd == 3 {   //Output, then input
                    let outfile = cargs[cargs.len() - 1].to_str();
                    crate::IORedirection::IORedirection::overwrite(outfile.unwrap());
                    cargs.pop();
                    cargs.pop();

                    let infile = cargs[cargs.len() - 1].to_str();
                    crate::IORedirection::IORedirection::readFile(infile.unwrap());
                    cargs.pop();
                    cargs.pop();
                }
                else if rd == 4 {   //Input, then output
                    let infile = cargs[cargs.len() - 1].to_str();
                    crate::IORedirection::IORedirection::readFile(infile.unwrap());
                    cargs.pop();
                    cargs.pop();

                    let outfile = cargs[cargs.len() - 1].to_str();
                    crate::IORedirection::IORedirection::overwrite(outfile.unwrap());
                    cargs.pop();
                    cargs.pop();
                }
		for i in 0..cargs2.len() {
		    cargs[0] = cargs2[i].to_owned();
		    execv(&cargs[0], &cargs);            
//                    unsafe { libc::_exit(0) };       ------May need to look back into this but for now it works -----
	    	}
            println!("failed to execute");
            0 as pid_t
	    }
            Err(_) => {
            println!("Forking Failed");
            0 as pid_t
        } 
    }

}
    //pub fn background_manager() -- to be used in order to clean up main
}