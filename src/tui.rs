use crossterm::event::{self, Event};

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

use crate::{Result, app::App};

pub fn tui_loop(app: &App) -> Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|meh| draw(meh, app))?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame, app: &App) {
    let mut controls_lines = vec![];
    let controls_txts = [
        "Use j/k to scroll the lore. Use e to enter edit mode to define your sql.",
        "Use t to enter talk mode. Use [enter] to test your solution.",
    ];
    for controls_txt in controls_txts {
        controls_lines.push(Line::from(controls_txt));
    }

    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(3), Min(0), Length(4), Length(3)]);
    let [title_area, main_area, controls_area, bottom_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [lore_area, output_area] = horizontal.areas(main_area);

    let title_block = Block::bordered().title(Line::from("STAGE").centered());
    let instructions_block = Block::bordered().title("INSTRUCTIONS");
    let lore_block = Block::bordered().title("LORE");
    let controls_block = Block::bordered().title(Line::from("").centered());
    let bottom_block = Block::bordered().title(Line::from("CPU").centered());

    // let input = "# Heading\n\n**bold**";
    let inst = app.instructions.clone();
    let markdown = tui_markdown::from_str(&inst);

    let lore_text = Paragraph::new(app.lore.clone())
        .block(lore_block)
        .wrap(Wrap { trim: true });
    let title_text = Paragraph::new(app.level.clone())
        .block(title_block)
        .centered()
        .wrap(Wrap { trim: true });

    // let instructions_text = Paragraph::new(markdown)
    //     .block(instructions_block)
    //     .wrap(Wrap { trim: true });

    // let instructions_text = Paragraph::new(app.instructions.clone())
    //     .block(instructions_block)
    //     .wrap(Wrap { trim: true });

    let instructions_text = Paragraph::new(app.output.clone())
        .block(instructions_block)
        .wrap(Wrap { trim: true });

    let controls_text = Paragraph::new(controls_lines)
        .block(controls_block)
        .centered()
        .wrap(Wrap { trim: true });

    frame.render_widget(title_text, title_area);
    frame.render_widget(instructions_text, output_area);
    frame.render_widget(lore_text, lore_area);
    frame.render_widget(controls_text, controls_area);

    frame.render_widget(bottom_block, bottom_area);
}
