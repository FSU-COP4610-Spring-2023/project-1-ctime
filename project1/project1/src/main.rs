use std::io::{self, Write};
use std::process::Command;

fn main(){
    loop {
        //char to use as prompt
        let prompt_char = "> ";
        
        //flush to ensure it prints before read_line
        print!("{prompt_char}");
        io::stdout().flush().ok();

        //Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        //Trim into tokens
        let mut tokens = input.trim().split_whitespace();
        let mut args = Vec::new();

        //Separate tokens into command and args
        let command= tokens.next().unwrap();
        for token in tokens{
            args.push(token);
        }

        //Spawn a new thread and execute the command with arguments
        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();

        child.wait().ok();
    }
}
