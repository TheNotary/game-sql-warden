pub struct App {
    pub level: String,
    pub output: String,
    pub lore: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            level: String::new(),
            output: String::new(),
            lore: String::new(),
        }
    }
}
