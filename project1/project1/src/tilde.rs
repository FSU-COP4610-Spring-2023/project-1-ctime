pub mod tilde {
    pub fn replace(mut input: String) -> String {
        input.remove(0);
        let mut value = std::env::var("HOME").expect("{input} is not set");
        value.push_str(&input);
        return value;
    }
}