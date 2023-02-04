#![allow(non_snake_case)]

use std::io::{self, Write};


mod prompt;
use prompt::prompt::print as printPrompt;

mod envVar;
use envVar::envVar::replace as replaceEnv;

mod tilde;
use tilde::tilde::replace as replaceTilde;

mod IORedirection;




mod execution;

use execution::execution::execute;

mod psearch;
use psearch::psearch::path_search;



use nix::unistd::Pid;
use nix::sys::wait::WaitStatus;
use nix::sys::wait::waitpid;
use nix::errno::Errno;



use std::env;
use::libc::pid_t;

mod backgroundExecute;
use backgroundExecute::backgroundExecute::background_execute;



use std::time::Duration;

fn main(){
  
    let path_var = env::var("PATH").unwrap_or("".to_string());

    let path_vars_vec: Vec<&str> = path_var.split(':').collect();

    let mut jobs: Vec<pid_t> = Vec::new();
    //let mut active_jobs: Vec<i32> = Vec::new();
    let mut job: i32 = 0;
    let mut jobs_complete: Vec<i32> = Vec::new();
    let mut saved_args:Vec<Vec<String>> = Vec::new();
    loop {
        let mut rdNum = 0;  //Passing in IOredirection behavior as an int
        let mut jobs_delete: Vec<i32> = Vec::new();

        //if there is a job running in background, loop through the list
        //of jobs and check their status. If finished remove from list.
        if!jobs.is_empty() 
        {
                for i in (0..(jobs.len()))
                {
                    if(jobs[i] != 0){
                        // println!("{}", jobs[i]);
                        // io::stdout().flush().expect("flush failure");

                    if procinfo::pid::stat(jobs[i]).expect("error finding pid").state == procinfo::pid::State::Zombie
                    {
                    if Ok::<WaitStatus, Errno>(waitpid(nix::unistd::Pid::from_raw(jobs[i]), None).expect("error"))
                    == Ok::<WaitStatus, Errno>(WaitStatus::Exited(nix::unistd::Pid::from_raw(jobs[i]), 0))
                        {
                            print!("[{}]+ [", jobs_complete[i]);
                            for n in 0..(saved_args[i].len()) 
                            {
                                print!(" {} ", saved_args[i][n]);
                            }
                            // println!("pushing: {}, pid: {}", i, jobs[i]);
                            jobs_delete.push(jobs[i]);
                            println!("]");
                            io::stdout().flush().expect("flush failure");
                        }
                    }
                }
                }
                //remove job from array when complete.
                if (jobs_delete.len() > 0){
                for i in 0..(jobs_delete.len()) {
                       
                    for n in 0..(jobs.len()) 
                     {
                        if (jobs[n] == jobs_delete[i]) {
                            jobs.remove(n);
                            saved_args.remove(n);
                            jobs_complete.remove(n);
                            io::stdout().flush().expect("flush failure");
                            break;
                        }
                    }
                }
                if (jobs.is_empty())
                {
                    job = 0;
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
	let tokens = input.split_whitespace();

	//create vectors to hold the strings and CStrings
	let mut args = Vec::new();

	//psuh each string into args vector
        for token in tokens {
            args.push(token.to_string());
        }

	//loop through the strings to find if any command 
	//is environment variable, ~ expansion, or io redirection
	for i in 0..args.len() {
            //Replace environment variables
            if args[i].starts_with('$') {
                args[i] = replaceEnv(args[i].to_string());
            }
            //Replace tilde
            if args[i].starts_with('~') {
                args[i] = replaceTilde(args[i].to_string());
            }
            //Assign int for redirection behavior
            if args[i] == ">" {
                if rdNum == 2 {
                    rdNum = 3;
                }
                else {
                    rdNum = 1;
                }              
            }
            else if args[i] == "<" {
                if rdNum == 1 {
                    rdNum = 4;
                }
                else {
                    rdNum = 2;
                }               
            }

            //jobs background functions.
            if args[i] == "jobs" {
                if jobs.len() >= 0 {
                    for i in 0..jobs.len() {
                    print!("[{}]+ [{}]", i+1, jobs[i]);
                    for n in 0..(saved_args[i].len()) {
                        print!(" {} ", saved_args[i][n]);
                        }
                        println!("]");
                    }
                    
                }
                else
                {
                    println!("No Jobs Active");

                }
            }
            io::stdout().flush().ok();

        }

	//create vec to hold the return of path_search (vector of command added to 
	//to the end of each directory path 
	let pvec: Vec<String> = path_search(&path_vars_vec, &args);

    //executes the process in background, storing a job number, 
    //and pushes the process id returned from execution into 
    //a vectore of process ids called jobs.
    
    let small_time = Duration::new(0, 5000000);
    if args[args.len() - 1] == "&"
    {
        job += 1;
        jobs_complete.push(job);
        print!("[{}] ",job);
        args.pop();
        saved_args.push(args.clone());
        let pid: pid_t = background_execute(args, pvec, rdNum);
        std::thread::sleep(small_time);
        jobs.push(pid);
        
    }
    
    //executes normally
    else
    {
	    execute(args, pvec,rdNum);

    }
}
}

