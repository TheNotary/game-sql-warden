use ratatui::widgets::ScrollbarState;

#[derive(Default)]
pub struct App {
    pub level: String,
    pub output: String,
    pub lore: String,
    pub instructions: String,
    pub left_pane_scroll: usize,
    pub left_pane_scroll_state: ScrollbarState,
    pub left_pane_mode: LeftPaneMode,
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
        self.left_pane_scroll = self.left_pane_scroll.saturating_sub(1);
        self.left_pane_scroll_state = self.left_pane_scroll_state.position(self.left_pane_scroll);
    }

    pub(crate) fn scroll_down(&mut self) {
        self.left_pane_scroll = self.left_pane_scroll.saturating_add(1);
        self.left_pane_scroll_state = self.left_pane_scroll_state.position(self.left_pane_scroll);
    }

    pub(crate) fn cycle_left_pane(&mut self) {
        self.left_pane_mode = self.left_pane_mode.next();
    }
}

#[derive(Default, Clone, Copy)]
pub enum LeftPaneMode {
    #[default]
    Lore,
    Instructions,
}

impl LeftPaneMode {
    pub fn next(self) -> Self {
        match self {
            LeftPaneMode::Lore => LeftPaneMode::Instructions,
            LeftPaneMode::Instructions => LeftPaneMode::Lore,
        }
    }
}
