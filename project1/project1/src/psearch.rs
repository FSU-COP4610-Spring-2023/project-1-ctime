pub mod psearch {
    //$PATH seargc function that takes the vector holding the command prompted and appends it
    //to the end of each directory path
    pub fn path_search(path_vec: &Vec<&str>, command_vec: &Vec<String>) -> Vec<String>{
		let mut temp_vec = Vec::new();

		for i in 0..path_vec.len() {
			let test: String = String::from(path_vec[i]);
			let test2: String = test + "/" + &command_vec[0];
			temp_vec.push(test2);
		}

		temp_vec	
    }
}
