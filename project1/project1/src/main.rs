#![allow(non_snake_case)]

use std::io::{self, Write};
use std::process::Command;



mod prompt;
use prompt::prompt::print as printPrompt;

mod envVar;
use envVar::envVar::replace as replaceEnv;

mod tilde;
use tilde::tilde::replace as replaceTilde;


fn main(){
    loop {
        //char to use as prompt
        // let prompt_char = "> ";
        //let prompt_str : &str = "";


        
        //flush to ensure it prints before read_line
        //print!("{prompt_char}");
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
    }
}
