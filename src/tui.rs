use ratatui::{
    crossterm::event::{self, Event, KeyEvent, KeyCode},
    DefaultTerminal,
    prelude::{Buffer, Frame, Rect},
    style::{Color, Stylize},
    layout::Alignment,
    widgets::{StatefulWidget, Block, List, ListItem, ListState},
};
use crate::note::Folder;

pub fn run(library: &Folder) -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    draw(&mut terminal, library)?;
    ratatui::restore();
    Ok(())
}

fn draw(terminal: &mut DefaultTerminal, folder: &Folder) -> std::io::Result<()> {
    let mut list_state = ListState::default().with_selected(Some(0));
    loop {
        terminal.draw(|frame| frame.render_stateful_widget(folder, frame.area(), &mut list_state))?;
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event {
                KeyEvent {code: KeyCode::Up, ..} => { list_state.select_previous(); },
                KeyEvent {code: KeyCode::Down, ..} => { list_state.select_next(); },
                KeyEvent {code: KeyCode::Char('k'), ..} => { list_state.select_previous(); },
                KeyEvent {code: KeyCode::Char('j'), ..} => { list_state.select_next(); },
                _ => {
                    break;
                },
            }
        }
    }
    let idx = list_state.selected().unwrap_or_else(|| panic!("No item selected"));
    if let Some(note) = folder.notes.get(idx) {
        note.edit("nvim");
    }

    if let Some(folder) = folder.folders.get(idx - folder.notes.len()) {
        draw(terminal, folder)?;
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
            |(idx, item)| if idx == state.selected().unwrap() {item.fg(Color::Red).rapid_blink()} else {item.fg(Color::DarkGray)}
        );

        let block = Block::bordered()
            .title_top("Notes")
            .title_alignment(Alignment::Center)
            .red()
            .bold()
            .on_white();
        let list = List::new(items).block(block);
        StatefulWidget::render(list, area, buf, state)
    }

    type State = ListState;
}
