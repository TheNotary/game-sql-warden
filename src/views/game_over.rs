pub fn tomb_stone() -> Vec<Vec<char>> {
    const TOMB_STONE: &[&str] = &[
        "                                      ",
        "            #################         ",
        "          #                   #       ",
        "        #                       #     ",
        "       #        GAME OVER        #    ",
        "      #                           #   ",
        "     #                             #  ",
        "     #     Index out of Bounds     #  ",
        "     #                             #  ",
        "     #   You attempted to access   #  ",
        "     #     an element that was     #  ",
        "     #    outside the bounds of    #  ",
        "     #   the array being indexed   #  ",
        "     #                             #  ",
        "     #                             #  ",
        "     #          TRY AGAIN          #  ",
        "     #                             #  ",
        "     #                             #  ",
        "     ###############################  ",
        "                                      ",
    ];
    TOMB_STONE.iter().map(|row| row.chars().collect()).collect()
}
