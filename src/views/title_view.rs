use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Clear, List, ListState, Paragraph, Wrap};

use crate::app::App;

pub fn draw_title_view(frame: &mut Frame<'_>, app: &mut App, title_state: &mut ListState) {
    use Constraint::{Length, Min};
    let vertical = Layout::vertical([Length(3), Length(8), Min(0), Length(4)]);

    let [title_area, logo_section, main_area, _legend_area] = vertical.areas(frame.area());

    let horizontal = Layout::horizontal([Min(0), Length(48), Min(0)]);

    let [_, logo_area, _] = horizontal.areas(logo_section);

    let title_text = Paragraph::new("SQL Warden".to_string())
        .block(Block::bordered())
        .centered()
        .wrap(Wrap { trim: true });
    frame.render_widget(title_text, title_area);

    let logo = "
╭━━━┳━━━┳╮╱╱╱╱╱╭╮╱╱╱╱╱╭╮╱╱╱╱╱╱╱╱╱╱╱╭╮╭╮   ╭━━━━╮
┃╭━╮┃╭━╮┃┃╱╱╱╱╱┃┃╱╱╱╱╱┃┃╱╱╱╱╱╱╱╱╱╱╭╯╰┫┃   ╰━┫┣━╯
┃╰━━┫┃╱┃┃┃╱╱╱╱╱┃┃╱╱╭━━┫╰━┳╮╱╭┳━┳┳━╋╮╭┫╰━╮   ┃┃
╰━━╮┃┃╱┃┃┃╱╭╮╱╱┃┃╱╭┫╭╮┃╭╮┃┃╱┃┃╭╋┫╭╮┫┃┃╭╮┃   ┃┃
┃╰━╯┃╰━╯┃╰━╯┃╱╱┃╰━╯┃╭╮┃╰╯┃╰━╯┃┃┃┃┃┃┃╰┫┃┃┃   ┃┃
╰━━━┻━━╮┣━━━╯╱╱╰━━━┻╯╰┻━━┻━╮╭┻╯╰┻╯╰┻━┻╯╰╯ ╭━┫┣━╮
╱╱╱╱╱╱╱╰╯╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╭━╯┃             ╰━━━━╯
╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╱╰━━╯ Powered by Ratatui"
        .trim();

    let logo_text = Paragraph::new(logo.to_string());
    frame.render_widget(logo_text, logo_area);

    // let main_text = Paragraph::new("Main Area".to_string())
    //     .block(Block::bordered())
    //     .centered()
    //     .bg(Color::Gray)
    //     .wrap(Wrap { trim: true });
    // frame.render_widget(main_text, main_area);

    let verb = "Start";

    let items = [
        format!("{verb} Game (Light Mode)"),
        format!("{verb} Game (Dark Mode)"),
        format!("Reset Database(s)"),
        format!("About"),
        format!("Quit"),
    ];

    let list = List::new(items)
        .block(Block::bordered().title("List"))
        .highlight_style(Style::new().italic())
        .highlight_symbol("-> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, main_area, title_state);

    if app.show_popup {
        let popup_area = popup_area(main_area, 60, 20);

        let popup_text = Paragraph::new(app.popup_text.clone())
            .block(Block::bordered())
            .centered()
            .bg(Color::Gray)
            .wrap(Wrap { trim: true });

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_text, popup_area);
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
