pub struct App {
    pub input: String,
    pub guesses: Vec<String>,
    pub last_return: String,
    pub return_code: u16,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            guesses: Vec::new(),
            last_return: String::new(),
            return_code: 0,
        }
    }
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            guesses: Vec::new(),
            last_return: String::new(),
            return_code: 0,
        }
    }
}
