use std::{fs, path::Path};

use ratatui::widgets::ScrollbarState;

use crate::{
    GameState, INSTRUCTIONS_PATH, LORE_PATH, NAME_PATH, Result, SOLUTION_PATH,
    api::{
        ActionToTake, ChallengeError, assess_db_condition, clear_level_in_sqlite, execute_solution,
        handle_db_condition, read_challenge_name, read_instructions_file, read_lore_file,
        read_solution_file,
    },
};

#[derive(Default)]
pub struct Stage {
    pub base_dir: String,
    pub id: u32,
    pub level: String,
    pub output: String,
    pub solution: String,
    pub lore: String,
    pub instructions: String,
}

impl Stage {
    pub fn new(
        id: u32,
        level: String,
        lore: String,
        instructions: String,
        solution: String,
        base_dir: String,
    ) -> Self {
        Self {
            id,
            level,
            lore,
            instructions,
            solution,
            base_dir,
            ..Default::default()
        }
    }

    pub(crate) fn from_dir(base_dir: &str) -> Self {
        let level = read_challenge_name(&format!("{base_dir}/{NAME_PATH}"));
        let lore = read_lore_file(&format!("{base_dir}/{LORE_PATH}"));
        let instructions = read_instructions_file(&format!("{base_dir}/{INSTRUCTIONS_PATH}"));
        let solution = read_solution_file(&format!("{base_dir}/{SOLUTION_PATH}"));
        let id = match get_stage_id(base_dir) {
            Ok(id) => id,
            Err(_) => 0,
        };

        Self::new(
            id,
            level,
            lore,
            instructions,
            solution,
            base_dir.to_string(),
        )
    }
}

#[derive(Default)]
pub struct App {
    pub stage: Stage,
    pub game_state: GameState,
    pub left_pane_scroll: usize,
    pub left_pane_scroll_state: ScrollbarState,
    pub left_pane_mode: LeftPaneMode,
    pub right_pane_mode: RightPaneMode,
    pub current_view: View,
    pub maze: Vec<Vec<char>>,
}

impl App {
    pub fn new(stage: Stage, game_state: GameState) -> Self {
        let map = vec![
            "                                      ".chars().collect(),
            "                              #       ".chars().collect(),
            "             ##  ##           #       ".chars().collect(),
            "            ##  # #           #       ".chars().collect(),
            "          ###############     #       ".chars().collect(),
            "          # 1  2        #### #        ".chars().collect(),
            "        #####  #####      ###         ".chars().collect(),
            "       ##  7#     3#       5#         ".chars().collect(),
            "            # ######### #####         ".chars().collect(),
            "            #6#       #4#             ".chars().collect(),
            "                                      ".chars().collect(),
            "                                      ".chars().collect(),
        ];

        Self {
            stage,
            game_state,
            maze: map,
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

    pub(crate) fn cycle_view_to_map(&mut self) {
        self.current_view = View::MapScreen;
    }

    pub(crate) fn cycle_view_to_challenge(&mut self) {
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
            self.current_view = View::NoStage;
        }
    }

    pub fn get_char_under_player(&self) -> char {
        let (r, c) = self.game_state.player;
        self.maze[r][c]
    }

    pub(crate) fn execute_solution(&self) -> Result<()> {
        let base_dir = &self.stage.base_dir;
        execute_solution(base_dir)?;
        Ok(())
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

// this is soooo bad still
fn get_stage_id(base_path: &str) -> Result<u32> {
    if let Some(challenge_dir) = base_path.strip_prefix("challenges/") {
        let digits: String = challenge_dir.chars().take(2).collect();

        if digits.len() == 2 {
            if let Ok(stage_id) = digits.parse::<u32>() {
                return Ok(stage_id);
            }
        }
    }

    Err(ChallengeError::InvalidChallengeDir(base_path.into()))
}

#[derive(Default, Clone, Copy)]
pub enum View {
    #[default]
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
