pub mod execution {
    use std::io::{self, Write};

    use std::ffi::CString;
    use std::os::raw::c_char;

    use nix::unistd::execv;
    use nix::{sys::wait::waitpid, unistd::{fork, ForkResult, write}};

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

    pub fn execute(mut input: Vec<String>) {
        let mut cargs = Vec::new();
        for i in input {
            let cstring = CString::new(i).unwrap();
            cargs.push(cstring);
        }

	match unsafe{fork()} {
            Ok(ForkResult::Parent { child, ..}) => {
                waitpid(child , None).unwrap();
            }
            Ok(ForkResult::Child) => {
                execv(&cargs[0], &cargs);
                unsafe { libc::_exit(0) };
            }
            Err(_) => println!("Forking Failed"),
        } 
    }

}
