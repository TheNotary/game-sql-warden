use std::collections::HashSet;

use crate::views::game_over::tomb_stone;

#[derive(Default)]
pub struct GameState {
    pub player: (usize, usize),
    pub cleared_levels: HashSet<u32>,
    pub maze: Vec<Vec<char>>,
}

impl GameState {
    pub fn new(player: (usize, usize), cleared_levels: HashSet<u32>) -> Self {
        Self {
            player,
            cleared_levels,
            maze: Self::default_maze(),
        }
    }

    pub fn default() -> Self {
        let cleared_levels = HashSet::new();
        let player = (5, 12);

        Self {
            cleared_levels,
            player,
            maze: Self::default_maze(),
        }
    }

    pub fn default_maze() -> Vec<Vec<char>> {
        const MAZE: &[&str] = &[
            "                                      ",
            "                              #       ",
            "             ##  ##           #       ",
            "            ##  # #           #       ",
            "          ###############     #       ",
            "          # 1  2        #### #        ",
            "        #####  #####      ###         ",
            "       ##  7#     3#       5#         ",
            "            # ######### #####         ",
            "            #6#       #4#             ",
            "                                      ",
            "                                      ",
        ];
        MAZE.iter().map(|row| row.chars().collect()).collect()
    }

    // Moves the player to loc as long as that location is within the bounds
    // of the maze and not a '#' symbol
    pub fn set_player(&mut self, loc: (usize, usize)) {
        let r = loc.0;
        let c = loc.1;
        let max_row = self.maze.len() - 1;
        let max_col = self.maze[0].len() - 1;

        let r_new = r.clamp(0, max_row);
        let c_new = c.clamp(0, max_col);

        if r_new != r || c_new != c {
            ratatui::restore();
            let tomb_stone = tomb_stone();
            for row in &tomb_stone {
                let line: String = row.iter().collect();

                println!("{line}");
            }
            std::process::exit(1);
        }

        if self.maze[r_new][c_new] != '#' {
            self.player = (r_new, c_new);
        }
    }
}
