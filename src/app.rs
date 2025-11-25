use ratatui::widgets::ScrollbarState;

#[derive(Default)]
pub struct App {
    pub level: String,
    pub output: String,
    pub lore: String,
    pub instructions: String,
    pub lore_scroll: usize,
    pub lore_scroll_state: ScrollbarState,
}

impl App {
    pub fn new(level: String, lore: String, instructions: String, output: String) -> Self {
        Self {
            level,
            lore,
            instructions,
            output,
            ..Default::default()
        }
    }

    pub(crate) fn scroll_up(&mut self) {
        self.lore_scroll = self.lore_scroll.saturating_sub(1);
        self.lore_scroll_state = self.lore_scroll_state.position(self.lore_scroll);
    }

    pub(crate) fn scroll_down(&mut self) {
        self.lore_scroll = self.lore_scroll.saturating_add(1);
        self.lore_scroll_state = self.lore_scroll_state.position(self.lore_scroll);
    }
}
