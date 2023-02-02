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

fn main(){
  
    let mut path_var = env::var("PATH").unwrap_or("".to_string());

    let mut path_vars_vec: Vec<&str> = path_var.split(":").collect();

    loop {
        let mut rdNum = 0;  //Passing in IOredirection behavior as an int

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
	//is environment variable, ~ expansion, or io redirection
	for i in 0..args.len() {
            //Replace environment variables
            if args[i].starts_with("$") {
                args[i] = replaceEnv(args[i].to_string());
            }
            //Replace tilde
            if args[i].starts_with("~") {
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
        }

	//create vec to hold the return of path_search (vector of command added to 
	//to the end of each directory path 
	let mut pvec: Vec<String> = path_search(&path_vars_vec, &args);
    
	execute(args, pvec, rdNum);
    }		
}

