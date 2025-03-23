use std::error::Error;

use ratatui::{
    crossterm::event::{self, Event, KeyEvent, KeyCode},
    DefaultTerminal,
    layout::{Layout, Constraint},
    prelude::{Buffer, Rect},
    style::{Color, Stylize},
    layout::Alignment,
    widgets::{Widget, StatefulWidget, Block, List, ListItem, ListState, Paragraph},
};
use crate::note::Folder;

pub fn run(library: &Folder) -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    draw(&mut terminal, library)?;
    ratatui::restore();
    Ok(())
}

fn draw(terminal: &mut DefaultTerminal, folder: &Folder) -> Result<(), Box<dyn Error>> {
    let mut list_state = ListState::default().with_selected(Some(0));
    loop {
        terminal.draw(|frame| frame.render_stateful_widget(folder, frame.area(), &mut list_state))?;
        if let Event::Key(key_event) = event::read()? {
            match key_event {
                KeyEvent {code: KeyCode::Up, ..} => { list_state.select_previous(); },
                KeyEvent {code: KeyCode::Down, ..} => { list_state.select_next(); },
                KeyEvent {code: KeyCode::Char('k'), ..} => { list_state.select_previous(); },
                KeyEvent {code: KeyCode::Char('j'), ..} => { list_state.select_next(); },
                KeyEvent {code: KeyCode::Enter, ..} => {
                    let idx = list_state.selected().ok_or("No item selected")?;
                    if let Some(note) = folder.notes.get(idx) {
                        ratatui::restore();
                        note.edit("nvim");
                        *terminal = ratatui::init();
                    } else if let Some(folder) = folder.folders.get(idx - folder.notes.len()) {
                        draw(terminal, folder)?;
                    }
                },
                _ => { break },
            }
        }
    }
    Ok(())
}

impl StatefulWidget for &Folder {
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let notes_items = self.notes.iter().map(
            |note| ListItem::new(note.title().to_owned()).bg(Color::Gray)
        );

        let folder_items = self.folders.iter().map(
            |folder| {
                ListItem::new(folder.title().to_owned()).bg(Color::LightYellow)
            }
        );

        let items = notes_items.chain(folder_items).enumerate().map(
            |(idx, item)| if idx == state.selected().unwrap() {item.fg(Color::Red)} else {item.fg(Color::DarkGray)}
        );

        let layout = Layout::default()
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ])
            .split(area);

        let instructions = Paragraph::new(
            "Use the up/down or j/k keys to navigate.
Select an item using the enter key."
        )
            .alignment(Alignment::Center)
            .dark_gray()
            .block(
                Block::bordered()
                    .title_top("Instructions")
                    .title_alignment(Alignment::Center)
                    .red()
                    .bold()
                    .on_white()
            );
        let list = List::new(items).block(
            Block::bordered()
                .title_top(self.title())
                .title_alignment(Alignment::Center)
                .red()
                .bold()
                .on_white()
        );
        Widget::render(instructions, layout[0], buf);
        StatefulWidget::render(list, layout[1], buf, state)
    }

    type State = ListState;
}
