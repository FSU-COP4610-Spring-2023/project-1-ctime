pub mod execution {
    use std::ffi::CString;
    use nix::unistd::execv;
    use nix::{sys::wait::waitpid, unistd::{fork, ForkResult}};
    use std::time::Duration;
    use std::os::unix::io::AsRawFd;
    use nix::unistd::close;

    pub fn execute(args1: Vec<String>, pvec1: Vec<String>, args2: Vec<String>, pvec2: Vec<String>, args3 : Vec<String>, pvec3 : Vec<String>, rd : i32, numPipes : i32) {	       
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
                    nix::unistd::dup2(pipe1.unwrap().1, std::io::stdout().as_raw_fd()).ok();
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
		            execv(&cargs1[0], &cargs1).ok();
                }
            }
            Err(_) => println!("Forking Failed"),
        } 
        close(pipe1.unwrap().1).ok();
        let pipe2 = nix::unistd::pipe();

        if numPipes > 0 {
            //piped process 1
            match unsafe{fork()} {
                Ok(ForkResult::Parent { child: _, ..}) => {
                    let sm_time = Duration::new(0,10000000);
                    std::thread::sleep(sm_time);
                }
                Ok(ForkResult::Child) => {
                    nix::unistd::dup2(pipe1.unwrap().0, std::io::stdin().as_raw_fd()).ok();
                    if numPipes == 2 {
                        nix::unistd::dup2(pipe2.unwrap().1, std::io::stdout().as_raw_fd()).ok();
                    }
                    for i in 0..cpvec2.len() {
                        cargs2[0] = cpvec2[i].to_owned();
                        execv(&cargs2[0], &cargs2).ok();
                    }
                    close(pipe1.unwrap().0).ok();
                    close(pipe1.unwrap().1).ok();
                }
                Err(_) => println!("Forking Failed"),
            }   
        }

        if numPipes == 2 {
            let sm_time = Duration::new(0,10000000);
            std::thread::sleep(sm_time);
            //piped process 2
            match unsafe{fork()} {
                Ok(ForkResult::Parent { child: _, ..}) => {
                    let sm_time = Duration::new(0,5000000);
                    std::thread::sleep(sm_time);
                }
                Ok(ForkResult::Child) => {
                    nix::unistd::dup2(pipe2.unwrap().0, std::io::stdin().as_raw_fd()).ok();
                    for i in 0..cpvec3.len() {
                        cargs3[0] = cpvec3[i].to_owned();
                        execv(&cargs3[0], &cargs3).ok();
                    }
                }
                Err(_) => println!("Forking Failed"),
            }   
        }
    }
}
