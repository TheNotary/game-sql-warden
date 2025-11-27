use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph, Wrap};

use crate::app::App;

pub fn draw_no_stage_view(frame: &mut Frame<'_>, _app: &mut App) {
    use Constraint::{Length, Min};

    //      Title
    // ----- Map ------
    // ----------------
    // ----------------
    //      Legend
    let vertical = Layout::vertical([Length(3), Min(0), Length(4)]);

    let [title_area, main_area, _legend_area] = vertical.areas(frame.area());

    let horizontal = Layout::horizontal([Length(100), Length(100), Length(100)]);
    let [_, main_area, _] = horizontal.areas(main_area);

    let title_block = Block::bordered();
    let title_text = Paragraph::new("NO STAGE".to_string())
        .block(title_block)
        .centered()
        .bg(Color::Gray)
        .wrap(Wrap { trim: true });

    frame.render_widget(title_text, title_area);

    let main_block = Block::bordered();
    let main_text = Paragraph::new("You exited the map screen when you were not on a stage.  You must stand on a number and exit the map screen.  Why do I let you exit the map screen when you don't have a stage selected?  I could ask you the same question because you're likely the one who wrote this very message! It matters not. Go back, be gone from this strange, liminal place.".to_string())
        .block(main_block)
        .centered()
        .bg(Color::Gray)
        .wrap(Wrap { trim: true });

    frame.render_widget(main_text, main_area);
}
