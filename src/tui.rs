use std::{
    io::stdin,
    error::Error,
};

use jiff::Zoned;
use ratatui::{
    crossterm::event::{self, Event, KeyEvent, KeyCode},
    DefaultTerminal,
    layout::{Layout, Constraint},
    prelude::{Buffer, Rect, Span},
    style::{Color, Stylize},
    layout::Alignment,
    widgets::{Widget, StatefulWidget, Block, Table, Row, Cell, TableState, Paragraph},
};
use crate::note::Folder;

impl Folder {

    pub fn run(&mut self, terminal: &mut DefaultTerminal, editor: &str) -> Result<bool, Box<dyn Error>> {
        let mut list_state = TableState::default().with_selected(Some(0));
        let mut continue_app = true;

        while continue_app {
            terminal.draw(|frame| frame.render_stateful_widget(&mut *self, frame.area(), &mut list_state))?;
            let Some(action) = Self::get_input() else {
                continue;
            };
            match action {
                MenuAction::ScrollUp => { list_state.select_previous() },
                MenuAction::ScrollDown => { list_state.select_next() },
                MenuAction::SelectItem => {
                    let idx = list_state.selected().unwrap_or(0);
                    if let Some(note) = self.notes.get(idx) {
                        ratatui::restore();
                        note.edit(editor);
                        *terminal = ratatui::init();
                    } else if let Some(folder) = self.folders.get_mut(idx - self.notes.len()) {
                        continue_app = folder.run(terminal, editor)?;
                    }
                },
                MenuAction::AddNote => {
                    ratatui::restore();

                    let stdin = stdin();
                    println!("Enter Title");
                    let mut title = String::new();
                    stdin.read_line(&mut title).expect("Error reading line");
                    title.retain(|c| !c.is_whitespace());

                    println!("Enter Author");
                    let mut author = String::new();
                    stdin.read_line(&mut author).expect("Error reading line");
                    author.retain(|c| !c.is_whitespace());

                    let date = Zoned::now();

                    self.add_note(&title, vec![], &author, &date)?;
                    *terminal = ratatui::init();
                },
                MenuAction::AddFolder => {
                    ratatui::restore();
                    println!("Enter Title");
                    let mut title = String::new();
                    stdin().read_line(&mut title).expect("Error reading line");
                    title.retain(|c| !c.is_whitespace());
                    self.add_folder(&title)?;

                    *terminal = ratatui::init();
                }
                MenuAction::Back => { break; },
                MenuAction::Quit => { return Ok(false); }
            }
        }
        Ok(true)
    }

    fn get_input() -> Option<MenuAction> {
        let Event::Key(key_event) = event::read().ok()? else {
            return None;
        };

        match key_event {
            KeyEvent {code: KeyCode::Up, ..} => Some(MenuAction::ScrollUp),
            KeyEvent {code: KeyCode::Char('k'), ..} => Some(MenuAction::ScrollUp),
            KeyEvent {code: KeyCode::Down, ..} => Some(MenuAction::ScrollDown),
            KeyEvent {code: KeyCode::Char('j'), ..} => Some(MenuAction::ScrollDown),
            KeyEvent {code: KeyCode::Enter, ..} => Some(MenuAction::SelectItem),
            KeyEvent {code: KeyCode::Char('l'), ..} => Some(MenuAction::SelectItem),
            KeyEvent {code: KeyCode::Backspace, ..} => Some(MenuAction::Back),
            KeyEvent {code: KeyCode::Char('h'), ..} => Some(MenuAction::Back),
            KeyEvent {code: KeyCode::Char('q'), ..} => Some(MenuAction::Quit),
            KeyEvent {code: KeyCode::Char('n'), ..} => Some(MenuAction::AddNote),
            KeyEvent {code: KeyCode::Char('N'), ..} => Some(MenuAction::AddFolder),
            _ => None,
        }
    }

}

enum MenuAction {
    ScrollUp,
    ScrollDown,
    SelectItem,
    AddNote,
    AddFolder,
    Back,
    Quit,
}

impl StatefulWidget for &mut Folder {
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let row_height = 2;
        let notes_items = self.notes.iter().map(
            |note| Row::new(vec![
                Cell::from(note.title()),
                Cell::from(note.author()),
                Cell::from(note.date().unwrap().to_string()),
            ]).height(row_height)
        );

        let folder_items = self.folders.iter().map(
            |folder| {
                Row::new(vec![
                    Span::from(folder.title()),
                ]).height(row_height)
            }
        );

        let titles = Row::new(vec![
            Span::from("Title"),
            Span::from("Author"),
            Span::from("Date"),
        ]).bg(Color::DarkGray).fg(Color::White).height(row_height);

        let items = notes_items.chain(folder_items).enumerate().map(
            |(idx, item)| if idx == state.selected().unwrap_or(0) {item.bg(Color::Gray).fg(Color::Black)} else {item}
        );
        let items = std::iter::once(titles).chain(items);

        let instructions_text = "Up: <j>, <Up>.
Select Item: <l>, <Enter>.
Go Up Level: <h>, <Backspace>
Quit: <q>
Add Note: <n>
Add Folder: <N>";
        let layout = Layout::default()
            .constraints(vec![
                Constraint::Min((instructions_text.lines().count() + 2) as u16),
                Constraint::Percentage(100),
            ])
            .split(area);

        let instructions = Paragraph::new(instructions_text)
            .alignment(Alignment::Center)
            .dark_gray()
            .block(
                Block::bordered()
                    .title_top("Instructions")
                    .title_alignment(Alignment::Center)
                    .gray()
                    .bold()
            );
        let widths: Vec<u16> = vec![];
        let list = Table::new(items, widths).block(
            Block::bordered()
                .title_top(self.title())
                .title_alignment(Alignment::Center)
                .gray()
                .bold()
        );
        Widget::render(instructions, layout[0], buf);
        StatefulWidget::render(list, layout[1], buf, state)
    }

    type State = TableState;
}
