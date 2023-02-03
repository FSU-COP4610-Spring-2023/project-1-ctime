#![allow(non_snake_case)]

use std::io::{self, Write};
use std::process::Command;

mod prompt;
use prompt::prompt::print as printPrompt;

mod envVar;
use envVar::envVar::replace as replaceEnv;

mod tilde;
use tilde::tilde::replace as replaceTilde;

mod commandSplit;
use commandSplit::commandSplit::getTokens;

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

        //Separate piped commands
        let mut commands : Vec<&str> = input.split("|").collect();
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

        //loop through the strings to find if any command 
        //is environment variable, ~ expansion, or io redirection
        for j in 0..argVec.len() {
            for i in 0..argVec[j].len() {
                //Replace environment variables
                if argVec[j][i].starts_with("$") {
                    argVec[j][i] = replaceEnv(argVec[j][i].to_string());
                }
                //Replace tilde
                if argVec[j][i].starts_with("~") {
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
        let mut pvec1: Vec<String> = path_search(&path_vars_vec, &argVec[0]);
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
        
        execute(args1, pvec1, args2, pvec2, args3, pvec3, rdNum, numPipes);
    }		
}

