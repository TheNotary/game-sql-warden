use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::App;

pub fn draw_map_view(frame: &mut Frame<'_>, app: &mut App) {
    use Constraint::{Length, Min};

    //      Title
    // ----- Map ------
    // ----------------
    // ----------------
    //      Legend
    let vertical = Layout::vertical([Length(3), Min(0), Length(4)]);

    let [title_area, map_area, _legend_area] = vertical.areas(frame.area());

    let title_block = Block::bordered();
    let title_text = Paragraph::new("MAP".to_string())
        .block(title_block)
        .centered()
        .bg(Color::Gray)
        .wrap(Wrap { trim: true });

    frame.render_widget(title_text, title_area);

    render_map(frame, map_area, app);
}

fn render_map(frame: &mut Frame<'_>, map_area: Rect, app: &App) {
    let text = process_maze_to_text(app);

    let paragraph = Paragraph::new(text)
        .centered()
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(paragraph, map_area);
}

fn process_maze_to_text(app: &App) -> String {
    let mut maze = app.game_state.maze.clone();
    let cleared_levels = &app.game_state.cleared_levels;

    // place player in maze
    let (r, c) = app.game_state.player;
    maze[r][c] = '@';

    // Format the maze as a string, replacing cleared_levels with a '*' symbol
    maze.iter()
        .map(|row| {
            row.iter()
                .map(|c| match c.to_digit(10) {
                    Some(d) if cleared_levels.contains(&d) => '*',
                    _ => *c,
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
