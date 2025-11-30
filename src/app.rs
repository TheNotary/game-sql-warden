use std::{fs, path::Path};

use log::debug;
use ratatui::widgets::ScrollbarState;

use crate::{
    Result, SOLUTION_PATH,
    api::{
        ActionToTake, assess_db_condition, clear_level_in_sqlite, execute_solution,
        handle_db_condition, read_solution_file,
    },
    game::{game_state::GameState, stage::Stage},
};

#[derive(Default)]
pub struct App {
    pub stage: Stage,
    pub game_state: GameState,
    pub left_pane_scroll: usize,
    pub left_pane_scroll_state: ScrollbarState,
    pub left_pane_mode: LeftPaneMode,
    pub right_pane_mode: RightPaneMode,
    pub current_view: View,
    pub show_popup: bool,
    pub popup_text: String,
    pub confirmation_task: Option<Box<dyn FnMut(&mut App)>>,
}

impl App {
    pub fn new(stage: Stage, game_state: GameState) -> Self {
        Self {
            stage,
            game_state,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_dir(base_dir: &str) -> App {
        let stage = Stage::from_dir(base_dir);
        let game_state = GameState::default();

        Self::new(stage, game_state)
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
        let base_dir = &self.stage.base_dir;

        // just print the cleared message if the database says wee've cleared it
        if self.game_state.cleared_levels.contains(&self.stage.id) {
            self.stage.output = String::from("🏆 You have cleared this stage!");
            return Ok(());
        }

        let action = handle_db_condition(assess_db_condition(base_dir)?)?;

        if let ActionToTake::LevelCleared(_) = action {
            let stage_id = self.stage.id;
            if !self.game_state.cleared_levels.contains(&stage_id) {
                self.game_state.cleared_levels.insert(stage_id);
                clear_level_in_sqlite(stage_id)?;
            }
        }

        self.stage.output = action.into_output();

        Ok(())
    }

    pub(crate) fn reload_solution_file(&mut self) {
        let base_dir = &self.stage.base_dir;
        self.stage.solution = read_solution_file(&format!("{base_dir}/{SOLUTION_PATH}"));
    }

    pub(crate) fn cycle_view_to_no_stage(&mut self) {
        debug!("cycling view to NoStage");
        self.current_view = View::NoStage;
    }

    pub(crate) fn cycle_view_to_map(&mut self) {
        debug!("cycling view to MapScreen");
        self.current_view = View::MapScreen;
    }

    pub(crate) fn cycle_view_to_challenge(&mut self) {
        debug!("cycling view to ChallengeScreen");
        self.current_view = View::ChallengeScreen;
    }

    pub(crate) fn update_current_stage(&mut self) {
        // check where player is
        // check character in map
        // if it's a number, load that stage
        // else, clear the stage

        let map_char = self.get_char_under_player();

        if map_char.is_numeric() {
            let level = map_char
                .to_digit(10)
                .expect("It was numeric butuuut....dudue");
            if let Some(base_dir) = get_path(level) {
                self.stage = Stage::from_dir(&base_dir);
            }
            self.cycle_view_to_challenge();
        } else {
            self.cycle_view_to_no_stage();
        }
    }

    pub fn get_char_under_player(&self) -> char {
        let (r, c) = self.game_state.player;
        self.game_state.maze[r][c]
    }

    pub(crate) fn execute_solution(&self) -> Result<()> {
        let base_dir = &self.stage.base_dir;
        execute_solution(base_dir)?;
        Ok(())
    }

    pub(crate) fn set_popup(&mut self, msg: &str, confirmation_task: Box<dyn FnMut(&mut App)>) {
        self.show_popup = true;
        self.popup_text = msg.to_string();
        self.confirmation_task = Some(confirmation_task);
    }
}

fn get_path(n: u32) -> Option<String> {
    let prefix = format!("{:02}_", n); // zero-pad to two digits + underscore
    let base = Path::new("challenges");

    // Iterate through entries in `challenges/`
    for entry in fs::read_dir(base).ok()? {
        let entry = entry.ok()?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.starts_with(&prefix) {
            return entry.path().to_str().and_then(|s| Some(s.to_string()));
        }
    }

    None
}

#[derive(Default, Clone, Copy)]
pub enum View {
    #[default]
    TitleScreen,
    ChallengeScreen,
    MapScreen,
    NoStage,
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
