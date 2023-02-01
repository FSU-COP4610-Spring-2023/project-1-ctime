pub mod psearch {

    //$PATH seargc function that takes the vector holding the command prompted and appends it
    //to the end of each directory path
    pub fn path_search(mut path_vec: &Vec<&str>, mut command_vec: &Vec<String>) -> Vec<String>{
	let mut temp_vec = Vec::new();

	for i in 0..path_vec.len() {
	    let mut test: String = String::from(path_vec[i]);
	    let mut test2: String = test + "/" + &command_vec[0];
	    temp_vec.push(test2);
	}

//	println!("{:?}", temp_vec);
	return temp_vec;	
    }
}
