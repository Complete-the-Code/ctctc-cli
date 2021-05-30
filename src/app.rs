pub struct App {
    pub input: String,
    pub guesses: Vec<String>,
    pub last_return: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            guesses: Vec::new(),
            last_return: String::new(),
        }
    }
}

impl App {
    pub fn new(guesses: Vec<String>) -> App {
        App {
            input: String::new(),
            guesses,
            last_return: String::new(),
        }
    }
}