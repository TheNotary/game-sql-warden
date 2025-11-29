use std::collections::HashSet;

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
}
