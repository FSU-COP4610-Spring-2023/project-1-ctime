pub mod execution {
    use std::io::{self};

    use std::ffi::CString;
    

    use nix::unistd::execv;
    use nix::{sys::wait::waitpid, unistd::{fork, ForkResult, write}};

    use std::time::Duration;

//    use std::os::fd::{AsRawFd, AsFd};
    use std::os::unix::io::AsRawFd;

    use std::env;

    use nix::unistd::close;

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
   
    pub fn convert_to_cstring(mut input: Vec<String>) -> Vec<CString> {
	let mut cargs = Vec::new();
	for i in input {
	    let cstring = CString::new(i).unwrap();
	    cargs.push(cstring);
	}	

	cargs
    }

    pub fn execute(mut args1: Vec<String>, mut pvec1: Vec<String>, mut args2: Vec<String>, mut pvec2: Vec<String>, mut args3 : Vec<String>, mut pvec3 : Vec<String>, rd : i32, numPipes : i32) {	       
        let pipe1 = nix::unistd::pipe();

        let mut cargs1 = Vec::new();
            for i in args1 {
                let cstring = CString::new(i).unwrap();
                cargs1.push(cstring);
            }

        let mut cpvec1 = Vec::new();
            for i in pvec1 {
                let cstring2 = CString::new(i).unwrap();
                cpvec1.push(cstring2);
            }

        let mut cargs2 = Vec::new();
        for i in args2 {
            let cstring2 = CString::new(i).unwrap();
            cargs2.push(cstring2);
        }

        let mut cpvec2 = Vec::new();
        for i in pvec2 {
            let cstring2 = CString::new(i).unwrap();
            cpvec2.push(cstring2);
        }

        let mut cargs3 = Vec::new();
        for i in args3 {
            let cstring3 = CString::new(i).unwrap();
            cargs3.push(cstring3);
        }

        let mut cpvec3 = Vec::new();
        for i in pvec3 {
            let cstring3 = CString::new(i).unwrap();
            cpvec3.push(cstring3);
        }

        
        match unsafe{fork()} {
            Ok(ForkResult::Parent { child, ..}) => {
                waitpid(child , None).unwrap();
            }
            Ok(ForkResult::Child) => {
                if numPipes > 0 {
                    nix::unistd::dup2(pipe1.unwrap().1, std::io::stdout().as_raw_fd());
                }
                if rd == 1 {    //1: Output
                    let outfile = cargs1[cargs1.len() - 1].to_str();
                    crate::IORedirection::IORedirection::overwrite(outfile.unwrap());
                    cargs1.pop();    //remove redirect token and output file from command vector
                    cargs1.pop();
                }
                else if rd == 2 {   //2: Input
                    let infile = cargs1[cargs1.len() - 1].to_str();
                    crate::IORedirection::IORedirection::readFile(infile.unwrap());
                    cargs1.pop();
                    cargs1.pop();
                }
                else if rd == 3 {   //Output, then input
                    let outfile = cargs1[cargs1.len() - 1].to_str();
                    crate::IORedirection::IORedirection::overwrite(outfile.unwrap());
                    cargs1.pop();
                    cargs1.pop();

                    let infile = cargs1[cargs1.len() - 1].to_str();
                    crate::IORedirection::IORedirection::readFile(infile.unwrap());
                    cargs1.pop();
                    cargs1.pop();
                }
                else if rd == 4 {   //Input, then output
                    let infile = cargs1[cargs1.len() - 1].to_str();
                    crate::IORedirection::IORedirection::readFile(infile.unwrap());
                    cargs1.pop();
                    cargs1.pop();

                    let outfile = cargs1[cargs1.len() - 1].to_str();
                    crate::IORedirection::IORedirection::overwrite(outfile.unwrap());
                    cargs1.pop();
                    cargs1.pop();
                }
                for i in 0..cpvec1.len() {
                    cargs1[0] = cpvec1[i].to_owned();
/*		    if execv(&cargs1[0], &cargs1).is_err() == true {
			println!("invalid command");
		    }else {
                        execv(&cargs1[0], &cargs1);
		    }*/
		    execv(&cargs1[0], &cargs1);
                    //execv(&cargs1[0], &cargs1);
                //unsafe { libc::_exit(0) };       ------May need to look back into this but for now it works -----
                }
            }
            Err(_) => println!("Forking Failed"),
        } 
        close(pipe1.unwrap().1);
        let pipe2 = nix::unistd::pipe();

        if numPipes > 0 {
            //piped process 1
            match unsafe{fork()} {
                Ok(ForkResult::Parent { child, ..}) => {
                    //waitpid(child , None).unwrap();
                    let sm_time = Duration::new(0,10000000);
                    std::thread::sleep(sm_time);
                }
                Ok(ForkResult::Child) => {
                    nix::unistd::dup2(pipe1.unwrap().0, std::io::stdin().as_raw_fd());
                    if numPipes == 2 {
                        nix::unistd::dup2(pipe2.unwrap().1, std::io::stdout().as_raw_fd());
                    }
                    for i in 0..cpvec2.len() {
                        cargs2[0] = cpvec2[i].to_owned();
                        execv(&cargs2[0], &cargs2);
                        //unsafe { libc::_exit(0) };
                    }
                    close(pipe1.unwrap().0);
                    close(pipe1.unwrap().1);
                }
                Err(_) => println!("Forking Failed"),
            }   
        }

        if numPipes == 2 {
            let sm_time = Duration::new(0,10000000);
            std::thread::sleep(sm_time);
            //piped process 2
            match unsafe{fork()} {
                Ok(ForkResult::Parent { child, ..}) => {
                    //waitpid(child , None).unwrap();
                    let sm_time = Duration::new(0,5000000);
                    std::thread::sleep(sm_time);
                }
                Ok(ForkResult::Child) => {
                    nix::unistd::dup2(pipe2.unwrap().0, std::io::stdin().as_raw_fd());
                    for i in 0..cpvec3.len() {
                        cargs3[0] = cpvec3[i].to_owned();
                        execv(&cargs3[0], &cargs3);
                        //unsafe { libc::_exit(0) };
                    }
                }
                Err(_) => println!("Forking Failed"),
            }   
        }
    }
}
