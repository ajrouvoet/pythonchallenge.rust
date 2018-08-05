pub struct Challenge {
    pub num: u8,
    pub url: String,
    pub hints: Vec<String>,
    pub solution_url: String
}

impl ToString for Challenge {
    fn to_string(&self) -> String {
        let title = format!(" Challenge {} ", self.num);
        let bar = String::from(format!("{:-^1$}\n", title, 80));
        let hints: String = self.hints.iter().map(|h| format!("  - {}\n", h)).collect();
        return bar
            + &format!("from: {}\n", self.url)
            + &format!("to: {}\n", self.solution_url)
            + &format!("via: \n{}", hints);
    }
}
