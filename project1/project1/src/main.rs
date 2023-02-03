#![allow(non_snake_case)]

use std::io::{self, Write};
use std::process::Command;

mod prompt;
use prompt::prompt::print as printPrompt;

mod envVar;
use envVar::envVar::replace as replaceEnv;

mod tilde;
use tilde::tilde::replace as replaceTilde;

mod IORedirection;
use IORedirection::IORedirection::overwrite;
use IORedirection::IORedirection::append;
use IORedirection::IORedirection::readFile;

mod execution;
use execution::execution::convert_to_cstring;
use execution::execution::execute;

mod psearch;
use psearch::psearch::path_search;

use std::ffi::CString;
use std::os::raw::c_char;

use nix::unistd::execv;
use nix::{sys::wait::waitpid, unistd::{fork, ForkResult, write}};

use std::env;
use::libc::pid_t;

mod backgroundExecute;
use backgroundExecute::backgroundExecute::background_execute;
use nix::unistd::Pid;
use procinfo::pid::status;

use std::time::Duration;

fn main(){
  
    let mut path_var = env::var("PATH").unwrap_or("".to_string());

    let mut path_vars_vec: Vec<&str> = path_var.split(":").collect();

    let mut jobs: Vec<pid_t> = Vec::new();
    let mut job: i32 = 0;
    let mut saved_args:Vec<Vec<String>> = Vec::new();
    let mut jobs_delete: Vec<usize> = Vec::new();
    loop {
        
        //if there is a job running in background, loop through the list
        //of jobs and check their status. If finished remove from list.
        if(jobs.len() > 0) {
            
            for i in 0..(jobs.len()) {
         
                if (procinfo::pid::stat(jobs[i]).unwrap().state != procinfo::pid::State::Running)
                {
                    print!("[{}]+ [", jobs[i]);
                    for n in 0..(saved_args[i].len()) {
                    print!(" {} ", saved_args[i][n]);
                    }
                    jobs_delete.push(i);
                    println!("]");
                    io::stdout().flush().expect("flush failure");

                }
                      
            }
        
        
        for i in 0..(jobs_delete.len()) {
            if(jobs.len() != 0){
            jobs.remove(jobs_delete[i]);
            saved_args.remove(jobs_delete[i]);
            job = job - 1;
            }
        }
           

    }
    
    
        

        //flush to ensure it prints before read_line
        printPrompt();
        io::stdout().flush().ok();
	
        //Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        

	//take in the string as tokens splitting whitespaces
	let mut tokens = input.trim().split_whitespace();

	//create vectors to hold the strings and CStrings
	let mut args = Vec::new();

	//psuh each string into args vector
        for token in tokens {
            args.push(token.to_string());
        }

	//loop through the strings to find if any command 
	//is environment variable or ~ expansion
	for i in 0..args.len() {
            //Replace environment variables
            if args[i].starts_with("$") {
                args[i] = replaceEnv(args[i].to_string());
            }
            //Replace tilde
            else if args[i].starts_with("~") {
                args[i] = replaceTilde(args[i].to_string());
            }
            //Check for redirection
            //Right now, just redirects the previous argument to the output file.  
            //Need to update to run the command and pass in the output.
            else if args[i] == ">" {
                overwrite(args[i-1].as_str(), args[i+1].as_str());
            }
            else if args[i] == ">>" {
                append(args[i-1].as_str(), args[i+1].as_str());
            }
            else if args[i] == "<" {
                let content = readFile(args[i-1].as_str());
            }

        }

	//create vec to hold the return of path_search (vector of command added to 
	//to the end of each directory path 
	let mut pvec: Vec<String> = path_search(&path_vars_vec, &args);

    //executes the process in background, storing a job number, 
    //and pushes the process id returned from execution into 
    //a vectore of process ids called jobs.
    let mut pid: i32 = 0;
    let small_time = Duration::new(0, 5000000);
    if (args[args.len() - 1] == "&")
    {
        job = job + 1;
        print!("[{}] ",job);
        args.pop();
        saved_args.push(args.clone());
        pid = background_execute(args, pvec);
        std::thread::sleep(small_time);
        jobs.push(pid);
        
    }
    else
    {
	    execute(args, pvec);

    }
}
}

