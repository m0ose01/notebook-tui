use ratatui::{
    crossterm::event::{self, Event, KeyEvent, KeyCode},
    prelude::{Buffer, Frame, Rect},
    style::Stylize,
    layout::Alignment,
    widgets::{StatefulWidget, Block, List, ListItem, ListState},
};
use crate::note::{Note, Folder};

pub fn run(library: &Folder) -> std::io::Result<usize> {
    let mut terminal = ratatui::init();
    let mut list_state = ListState::default().with_selected(Some(0));
    loop {
        terminal.draw(|f| draw(f, library, &mut list_state))?;
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event {
                KeyEvent {code: KeyCode::Up, ..} => { list_state.select_previous(); },
                KeyEvent {code: KeyCode::Down, ..} => { list_state.select_next(); },
                _ => {break},
            }
        }
    }
    ratatui::restore();
    Ok(list_state.selected().unwrap())
}

fn draw(frame: &mut Frame, folder: &Folder, mut selected: &mut ListState) {
    frame.render_stateful_widget(folder, frame.area(), &mut selected);
}

impl StatefulWidget for &Folder {
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let block = Block::bordered()
            .title_top("Notes")
            .title_alignment(Alignment::Center)
            .red()
            .bold()
            .on_white();
        let list = List::new(&self.notes).block(block);
        StatefulWidget::render(list, area, buf, state)
    }

    type State = ListState;
}

impl From<&Note> for ListItem<'_> {
    fn from(note: &Note) -> Self {
        let mut title = note.title().to_owned();
        title.insert_str(0, "    \u{1F4D1} ");
        Self::from(title).bold().red().on_gray()
    }
}
