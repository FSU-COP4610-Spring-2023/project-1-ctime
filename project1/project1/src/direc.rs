pub mod direc {
    use std::env;
    
    //command to determine the current working directory of the shell
    pub fn find_curr_direc() -> String {
        let dir = env::current_dir();
        match dir {
            Ok(path) => path.into_os_string().into_string().unwrap(),
            Err(_) => "FAILED".to_string()
        }
    }

    //command to change directory currently wokring in 
    pub fn change_dir(_input_string: String, input_args: &Vec<String>) {
	//set variable to $HOME directory for later use 
	//and call fin_curr_direc to find where shell is located rn
        let home_var = env::var("HOME").unwrap();
        let curr_dir: String = find_curr_direc();

	//split directory path into seperate values 
        let curr_dir_as_vec: Vec<&str> = curr_dir.split("/").collect();

	//check if cd is trying too many commands at once
        if input_args.len() > 2 {
            println!("Too many arguments. Try again.");
        }
	//if just cd command typed set directory to $HOME path
	else if input_args.len() == 1 {	
            env::set_current_dir(home_var).ok();
        }
	//if "cd .." typed move back on directory 
	else if input_args[1] == ".." {  
            let mut new_dir = String::new();
            new_dir = new_dir + "/";
            for i in 1..curr_dir_as_vec.len() {
                if i == curr_dir_as_vec.len() - 1 {
                    break;
                }
                new_dir = new_dir  + curr_dir_as_vec[i] + "/";
            }
            env::set_current_dir(new_dir).ok();
        }
	//if specific directory given set directry to that
	else {					
	    //check if directory is exists
	    //if it doesn't then tell user there was an error
            let new_dir1: String = curr_dir.to_owned() + &input_args[1];
            let new_dir2: String = curr_dir.to_owned() + "/" + &input_args[1];
            if env::set_current_dir(new_dir1).is_err() == true &&
                env::set_current_dir(new_dir2).is_err() == true {
                println!("directory does not exist");
            }
	    //check if user enter "/" at beginning of directory 
	    //if not add it so set_cirrent_dir works properly 
            else if &input_args[1].starts_with("/") != &true {
                let new_dir: String = curr_dir.to_owned() + "/" + &input_args[1];
                env::set_current_dir(new_dir).ok();
            }
            else if &input_args[1].starts_with("/") == &true{
                let new_dir: String = curr_dir.to_owned() + &input_args[1];
                env::set_current_dir(new_dir).ok();
            }
        }
    }
}
