use ratatui::style::{Color, Stylize};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
};

use crate::app::{App, LeftPaneMode, RightPaneMode};

pub(crate) fn draw_challenge_view(frame: &mut Frame<'_>, app: &mut App) {
    let mut controls_lines = vec![];
    let controls_txts = [
        // "‣ [tab] cycle lore/instructions.   ‣ [del] cycle output/solution.",
        "‣ [j/k] scroll the lore.           ‣ [.,/] enter sqlite shell.",
        "‣ [e] edit solution.sql            ‣ [enter] test your solution.",
    ];
    for controls_txt in controls_txts {
        controls_lines.push(Line::from(controls_txt));
    }

    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(3), Min(0), Length(4)]);
    let [title_area, main_area, controls_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [lore_area, output_area] = horizontal.areas(main_area);

    let (left_pane_title, left_pane_content) = match app.left_pane_mode {
        LeftPaneMode::Lore => ("LORE", app.lore.clone()),
        LeftPaneMode::Instructions => ("INSTRUCTIONS", app.instructions.clone()),
    };

    let (right_pane_title, right_pane_content) = match app.right_pane_mode {
        RightPaneMode::Output => ("OUTPUT", app.output.clone()),
        RightPaneMode::Solution => ("SOLUTION", app.solution.clone()),
    };

    let title_block = Block::bordered().title(Line::from("STAGE").centered());
    let left_pane_block = Block::bordered()
        .title(left_pane_title)
        .title_bottom(Line::from(" [tab] cycle lore/instructions ").centered());
    let right_page_block = Block::bordered()
        .title(right_pane_title)
        .title_bottom(Line::from(" [del] cycle output/solution. ").centered());
    let controls_block = Block::bordered().title(Line::from("CONTROLS").centered());

    let left_pane_text = Paragraph::new(left_pane_content)
        .block(left_pane_block)
        .wrap(Wrap { trim: true })
        .bg(Color::Gray)
        .scroll((app.left_pane_scroll as u16, 0));

    let right_pane_text = Paragraph::new(right_pane_content)
        .block(right_page_block)
        .bg(Color::Gray)
        .wrap(Wrap { trim: true });

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        lore_area,
        &mut app.left_pane_scroll_state,
    );

    let title_text = Paragraph::new(app.level.clone())
        .block(title_block)
        .centered()
        .bg(Color::Gray)
        .wrap(Wrap { trim: true });

    let controls_text = Paragraph::new(controls_lines)
        .block(controls_block)
        .bg(Color::Gray)
        .centered();

    frame.render_widget(title_text, title_area);
    frame.render_widget(right_pane_text, output_area);
    frame.render_widget(left_pane_text, lore_area);
    frame.render_widget(controls_text, controls_area);
}
