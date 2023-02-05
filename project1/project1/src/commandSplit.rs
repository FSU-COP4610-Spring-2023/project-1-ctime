pub mod commandSplit {
    pub fn getTokens(mut commands : Vec<&str>) -> (Vec<String>, Vec<String>, Vec<String>){
        let mut a1 = Vec::new();
        let mut a2 = Vec::new();
        let mut a3 = Vec::new();
        for i in 0..commands.len() {
            commands[i] = commands[i].trim();
        }

        let a1iter = commands[0].split_whitespace();
        for a in a1iter {
            a1.push(a.to_string());
        }
        if commands.len() > 1 {
            let a2iter = commands[1].split_whitespace();
            for a in a2iter {
                a2.push(a.to_string());
            }
        }
        if commands.len() > 2 {
            let a3iter = commands[2].split_whitespace();
            for a in a3iter {
                a3.push(a.to_string());
            }
        }
        return (a1, a2, a3);
    }
}