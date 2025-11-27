use ratatui::widgets::ScrollbarState;

use crate::{
    INSTRUCTIONS_PATH, LORE_PATH, NAME_PATH, Result, SOLUTION_PATH,
    api::{
        assess_db_condition, handle_db_condition, read_challenge_name, read_instructions_file,
        read_lore_file, read_solution_file,
    },
};

#[derive(Default)]
pub struct App {
    pub base_dir: String,
    pub level: String,
    pub output: String,
    pub solution: String,
    pub lore: String,
    pub instructions: String,
    pub left_pane_scroll: usize,
    pub left_pane_scroll_state: ScrollbarState,
    pub left_pane_mode: LeftPaneMode,
    pub right_pane_mode: RightPaneMode,
    pub current_view: View,
    pub map: Vec<Vec<char>>,
    pub player: (usize, usize),
}

impl App {
    pub fn new(
        level: String,
        lore: String,
        instructions: String,
        solution: String,
        base_dir: String,
    ) -> Self {
        let map = vec![
            "                        #  ".chars().collect(),
            "       ##  ##           #  ".chars().collect(),
            "      ##  # #           #  ".chars().collect(),
            "    ###############     #  ".chars().collect(),
            "    # 1  2        #### #   ".chars().collect(),
            "  #####  #####      ###    ".chars().collect(),
            " ##  7#     3#       5#    ".chars().collect(),
            "      # ######### #####    ".chars().collect(),
            "      #2#       #4#        ".chars().collect(),
            "                           ".chars().collect(),
        ];
        let player = (4, 6);

        Self {
            level,
            lore,
            instructions,
            solution,
            base_dir,
            map,
            player,
            ..Default::default()
        }
    }

    pub(crate) fn from_dir(base_dir: &str) -> App {
        let level = read_challenge_name(&format!("{base_dir}/{NAME_PATH}"));
        let lore = read_lore_file(&format!("{base_dir}/{LORE_PATH}"));
        let instructions = read_instructions_file(&format!("{base_dir}/{INSTRUCTIONS_PATH}"));
        let solution = read_solution_file(&format!("{base_dir}/{SOLUTION_PATH}"));

        Self::new(level, lore, instructions, solution, base_dir.to_string())
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

    pub(crate) fn cycle_right_pane(&mut self) {
        self.right_pane_mode = self.right_pane_mode.next();
    }

    pub(crate) fn assess_db(&mut self) -> Result<()> {
        let base_dir = &self.base_dir;
        self.output = handle_db_condition(assess_db_condition(base_dir)?)?;
        Ok(())
    }

    pub(crate) fn reload_solution_file(&mut self) {
        let mut solution_path = self.base_dir.to_string();
        solution_path.push_str(&SOLUTION_PATH);
        self.solution = read_solution_file(&solution_path);
    }

    pub(crate) fn cycle_view(&mut self) {
        self.current_view = self.current_view.next();
    }
}

#[derive(Default, Clone, Copy)]
pub enum View {
    #[default]
    ChallengeScreen,
    MapScreen,
}

impl View {
    pub fn next(self) -> Self {
        match self {
            View::ChallengeScreen => View::MapScreen,
            View::MapScreen => View::ChallengeScreen,
        }
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

#[derive(Default, Clone, Copy)]
pub enum RightPaneMode {
    #[default]
    Output,
    Solution,
}

impl RightPaneMode {
    pub fn next(self) -> Self {
        match self {
            RightPaneMode::Output => RightPaneMode::Solution,
            RightPaneMode::Solution => RightPaneMode::Output,
        }
    }
}
