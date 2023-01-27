pub mod envVar {
    pub fn replace(mut input: String) -> String {
        input.remove(0);

        match std::env::var(input) {
            Ok(val) => return val,
            Err(_e) => return "".to_string(),
        }
    }
}
