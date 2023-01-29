#![allow(non_snake_case)]

use std::io::{self, Write};
use std::process::Command;

mod prompt;
use prompt::prompt::print as printPrompt;

mod envVar;
use envVar::envVar::replace as replaceEnv;

mod tilde;
use tilde::tilde::replace as replaceTilde;

use std::ffi::CString;
use std::os::raw::c_char;

use nix::unistd::execv;
use nix::{sys::wait::waitpid, unistd::{fork, ForkResult, write}};

use std::env;

fn main(){
    loop {
        
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
	let mut cargs = Vec::new();

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
        }

	//push srtings into CString vector to pass into execv function
	for j in args {
	    let cstr = CString::new(j).unwrap();
	    cargs.push(cstr);
	}

	//Forking function - creat child process to run execv 
	//for command line functionality
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
	//started trying out execv 
//	execv(cargs[0], cargs);
//	println!("finished demo");
	
/*
        printPrompt();
        io::stdout().flush().ok();

        //Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        //Trim into tokens
        let mut tokens = input.trim().split_whitespace();
        let mut args = Vec::new();

        //Separate tokens into command and args
        let command= tokens.next().unwrap();
        for token in tokens {
            args.push(token.to_string());
        }
        
        for i in 0..args.len() {
            //Replace environment variables
            if args[i].starts_with("$") {
                args[i] = replaceEnv(args[i].to_string());
            }
            //Replace tilde
            else if args[i].starts_with("~") {
                args[i] = replaceTilde(args[i].to_string());
            }
        }

        //Spawn a new thread and execute the command with arguments
        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();

        child.wait().ok();
*/    
}
