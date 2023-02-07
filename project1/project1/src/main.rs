#![allow(non_snake_case)]
#![allow(unused_comparisons)]

use std::io::{self, Write};
mod prompt;
use prompt::prompt::print as printPrompt;

mod envVar;
use envVar::envVar::replace as replaceEnv;

mod tilde;
use tilde::tilde::replace as replaceTilde;

mod commandSplit;
use commandSplit::commandSplit::getTokens;

mod IORedirection;

mod execution;
use execution::execution::execute;

mod psearch;
use psearch::psearch::path_search;

use nix::sys::wait::WaitStatus;
use nix::sys::wait::waitpid;
use nix::errno::Errno;

mod echoFunc;
use echoFunc::echoFunc::echoCmd;

use std::env;
use::libc::pid_t;

mod backgroundExecute;
use backgroundExecute::backgroundExecute::background_execute;

use std::time::Duration;

mod direc;
use direc::direc::find_curr_direc;
use direc::direc::change_dir;

fn main(){
  
    let path_var = env::var("PATH").unwrap_or("".to_string());

    let path_vars_vec: Vec<&str> = path_var.split(':').collect();

    let mut jobs: Vec<pid_t> = Vec::new();
    let mut job: i32 = 0;
    let mut jobs_complete: Vec<i32> = Vec::new();
    let mut saved_args1:Vec<Vec<String>> = Vec::new();
    let mut saved_args2:Vec<Vec<String>> = Vec::new();
    let mut saved_args3:Vec<Vec<String>> = Vec::new();
    //previous command array
    let mut last_command:Vec<String> = Vec::new();
    //exit check
    let mut exit = false;
    loop {
        let mut rdNum = 0;  //Passing in IOredirection behavior as an int
        let mut jobs_delete: Vec<i32> = Vec::new();

        //if there is a job running in background, loop through the list
        //of jobs and check their status. If finished remove from list.
        if!jobs.is_empty() 
        {
            for i in 0..(jobs.len())
            {
                if jobs[i] != 0 {
                    if exit == true
                    {
                        waitpid(nix::unistd::Pid::from_raw(jobs[i]), None).ok();
                        jobs_delete.push(jobs[i]);
                    }
                    if procinfo::pid::stat(jobs[i]).expect("error finding pid").state == procinfo::pid::State::Zombie
                    {
                        if Ok::<WaitStatus, Errno>(waitpid(nix::unistd::Pid::from_raw(jobs[i]), None).expect("error"))
                        == Ok::<WaitStatus, Errno>(WaitStatus::Exited(nix::unistd::Pid::from_raw(jobs[i]), 0))
                        {
                            print!("[{}]+ [", jobs_complete[i]);
                            for n in 0..(saved_args1[i].len()) 
                            {
                                print!(" {} ", saved_args1[i][n]);
                            }
                            if !saved_args2.is_empty()
                            {
                                print!("|");
                                for n in 0..(saved_args2[i].len()) 
                                {
                                    print!(" {} ", saved_args2[i][n]);
                                }
                            }
                            if !saved_args3.is_empty()
                            {
                                print!("|");
                                for n in 0..(saved_args3[i].len()) 
                                {
                                    print!(" {} ", saved_args3[i][n]);
                                }
                            }
                            jobs_delete.push(jobs[i]);
                            println!("]");
                            io::stdout().flush().expect("flush failure");
                        }
                    }
                }
            }
            //remove job from array when complete.
            if jobs_delete.len() > 0 {
                for i in 0..(jobs_delete.len()) {                  
                    for n in 0..(jobs.len()) 
                    {
                        if jobs[n] == jobs_delete[i] {
                            jobs.remove(n);
                            saved_args1.remove(n);
                            jobs_complete.remove(n);
                            io::stdout().flush().expect("flush failure");
                            break;
                        }
                    }
                }
                if jobs.is_empty()
                {
                    job = 0;
                    saved_args1.clear();
                    saved_args2.clear();
                    saved_args3.clear();
                }
            }
        }
        if exit == true
        {
            unsafe {libc::exit(0)};
        }

        //flush to ensure it prints before read_line
        printPrompt();
        io::stdout().flush().ok();
	
        //Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();        
        if input != "exit\n"
        {
            last_command.push(input.clone());
        }
        //Separate piped commands
        let commands : Vec<&str> = input.split("|").collect();
        let numPipes = (commands.len() as i32) - 1;
        if commands.len() > 3 {
            println!("Too many piped commands");
            continue;
        }

        let (mut args1, mut args2, mut args3) = getTokens(commands);
        let mut argVec = Vec::new();
        argVec.push(args1);
        argVec.push(args2);
        argVec.push(args3);

        //added cd functionality here. calls change_dir function to switch directories
        if argVec[0][0] == "cd" {
            let _temp_string: String = find_curr_direc();
            change_dir(find_curr_direc(), &argVec[0]);
        }

        if argVec[0][0] == "jobs" {
            if jobs.len() >= 0 {
                for i in 0..jobs.len() {
                    print!("[{}]+ [{}] [", i+1, jobs[i]);
                    for n in 0..(saved_args1[i].len()) 
                    {
                        print!(" {} ", saved_args1[i][n]);
                    }
                    if !saved_args2.is_empty()
                    {
                        print!("|");
                        for n in 0..(saved_args2[i].len()) 
                        {
                            print!(" {} ", saved_args2[i][n]);
                        }
                    }
                    if !saved_args3.is_empty()
                    {
                        print!("|");
                        for n in 0..(saved_args3[i].len()) 
                        {
                            print!(" {} ", saved_args3[i][n]);
                        }
                    }
                    println!("]");
                }
            }
            io::stdout().flush().ok();           
        }
        if argVec[0][0] == "exit"
        {
            exit = true;
            if last_command.len() >= 3
            {
                println!("The following was the last three valid commands executed:");
                print!("1. {}", last_command.pop().unwrap());
                print!("2. {}", last_command.pop().unwrap());
                print!("3. {}", last_command.pop().unwrap());
            }
            else if !last_command.is_empty()
            {
                println!("The last valid command executed was:");
                println!("{}", last_command.pop().unwrap());
            }
            else
            {
                println!("No valid commands were executed in this shell");
            }
        }

        //loop through the strings to find if any command 
        //is environment variable, ~ expansion, or io redirection
        for j in 0..argVec.len() {
            for i in 0..argVec[j].len() {
                //Replace environment variables
                if argVec[j][i].starts_with('$') {
                    argVec[j][i] = replaceEnv(argVec[j][i].to_string());
                }
                //Replace tilde
                if argVec[j][i].starts_with('~') {
                    argVec[j][i] = replaceTilde(argVec[j][i].to_string());
                }
                //Assign int for redirection behavior
                if argVec[j][i] == ">" {
                    if rdNum == 2 {
                        rdNum = 3;
                    }
                    else {
                        rdNum = 1;
                    }              
                }
                else if argVec[j][i] == "<" {
                    if rdNum == 1 {
                        rdNum = 4;
                    }
                    else {
                        rdNum = 2;
                    }               
                }         
            }
        }

        //create vec to hold the return of path_search (vector of command added to 
        //to the end of each directory path 
        let pvec1: Vec<String> = path_search(&path_vars_vec, &argVec[0]);
        let mut pvec2 = Vec::new();
        let mut pvec3 = Vec::new();

        if numPipes > 0 {
            pvec2 = path_search(&path_vars_vec, &argVec[1]);
        }

        if numPipes > 1 {
            pvec3 = path_search(&path_vars_vec, &argVec[2]);
        }

        args1 = argVec[0].clone();
        args2 = argVec[1].clone();
        args3 = argVec[2].clone();
        

        //executes the process in background, storing a job number, 
        //and pushes the process id returned from execution into 
        //a vectore of process ids called jobs.
        let small_time = Duration::new(0, 5000000);
        
        if !args3.is_empty()
        {
            if args3[args3.len() - 1] == "&"
            {
                job += 1;
                jobs_complete.push(job);
                print!("[{}] ",job);
                args3.pop();
                io::stdout().flush().ok();
                saved_args1.push(args1.clone());
                saved_args2.push(args2.clone());
                saved_args3.push(args3.clone());
                let pid: pid_t = background_execute(args1, pvec1, args2, pvec2, args3, pvec3, rdNum, numPipes);
                std::thread::sleep(small_time);
                jobs.push(pid);      
            }
            else{
                execute(args1, pvec1, args2, pvec2, args3, pvec3, rdNum, numPipes);
            }
        }
        else if !args2.is_empty()
        {
            if args2[args2.len() - 1] == "&"
            {
                job += 1;
                jobs_complete.push(job);
                print!("[{}] ",job);
                args2.pop();
                io::stdout().flush().ok();
                saved_args1.push(args1.clone());
                saved_args2.push(args2.clone());
                let pid: pid_t = background_execute(args1, pvec1, args2, pvec2, args3, pvec3, rdNum, numPipes);
                std::thread::sleep(small_time);
                jobs.push(pid);      
            }
            else {
                execute(args1, pvec1, args2, pvec2, args3, pvec3, rdNum, numPipes);
            }
        }
        else if args1[args1.len() - 1] == "&"
        {
            job += 1;
        jobs_complete.push(job);
            print!("[{}] ",job);
            args1.pop();
            io::stdout().flush().ok();
            saved_args1.push(args1.clone());
            let pid: pid_t = background_execute(args1, pvec1, args2, pvec2, args3, pvec3, rdNum, numPipes);
            std::thread::sleep(small_time);
            jobs.push(pid);      
        }
        else if args1[0] == "echo" {
            echoCmd(&argVec[0]);
        }
        else
        {
            execute(args1, pvec1, args2, pvec2, args3, pvec3, rdNum, numPipes);
        }
    }
}

