pub mod echoFunc {

    //quick function to print back out the user statements for built in echo function 
    pub fn echoCmd(input_string: &Vec<String>){
        for i in 1..input_string.len(){
            print!("{} ", input_string[i]);
        }
        println!("");
    }
}
