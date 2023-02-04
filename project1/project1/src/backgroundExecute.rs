pub mod backgroundExecute{
 use std::io::{self, Write};

    use std::ffi::CString;
    use std::os::raw::c_char;

    use nix::unistd::execv;
    use nix::unistd::Pid;
    use nix::{sys::wait::waitpid, unistd::{fork, ForkResult, write}};
    use libc::pid_t;

    use std::env;
    

    pub fn fill_arg_vector() -> Vec<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let mut tokens = input.trim().split_whitespace();

        let mut args = Vec::new();

        for token in tokens {
	    args.push(token.to_string());
	}     

	return args;
    }

    
    pub fn convert_to_cstring(mut input: Vec<String>) -> Vec<CString> {
	let mut cargs = Vec::new();
	for i in input {
	    let cstring = CString::new(i).unwrap();
	    cargs.push(cstring);
	}	

	return cargs;
    }

    pub fn background_execute(mut input: Vec<String>, mut input2: Vec<String>) -> pid_t{	
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
                return i32::from(child);
            }
            Ok(ForkResult::Child) => {
		for i in 0..cargs2.len() {
		    cargs[0] = cargs2[i].to_owned();
		    execv(&cargs[0], &cargs);            
//                    unsafe { libc::_exit(0) };       ------May need to look back into this but for now it works -----
	    	}
            println!("failed to execute");
            return 0 as pid_t;
	    }
            Err(_) => {
            println!("Forking Failed");
            return 0 as pid_t;
        } 
    }

}
}