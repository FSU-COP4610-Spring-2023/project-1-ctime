pub mod envVar {
    pub fn replace(mut input: String) -> String {
        input.remove(0);

        match std::env::var(input) {
            Ok(val) => val,
            Err(_e) => "".to_string(),
        }
    }
}
