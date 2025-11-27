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
    let mut rendered = app.maze.clone();
    let (r, c) = app.player;
    rendered[r][c] = '@';

    let text: String = rendered
        .iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect();

    let paragraph = Paragraph::new(text)
        .centered()
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(paragraph, map_area);
}
