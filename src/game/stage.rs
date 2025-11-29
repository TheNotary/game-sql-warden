use crate::{
    INSTRUCTIONS_PATH, LORE_PATH, NAME_PATH, Result, SOLUTION_PATH,
    api::{
        ChallengeError, read_challenge_name, read_instructions_file, read_lore_file,
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
