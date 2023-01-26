pub mod envVar {
    pub fn replace(mut input: String) -> String {
        input.remove(0);
        let value = std::env::var(input).expect("{input} is not set");
        return value;
    }
}