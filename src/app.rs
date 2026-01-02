use std::io::BufReader;

use color_eyre::{Result, owo_colors::OwoColorize};
use ratatui::{
    DefaultTerminal, crossterm,
    prelude::*,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub enum Screen {
    StartMenu,
    TierList,
    UploadingImage,
}

pub struct TierListItem {
    text: String,
    area: Option<Rect>,
}

pub struct App {
    pub current_screen: Screen,
    quitting: bool,
    tier_list_state: TableState,
    items: Vec<TierListItem>,
}

#[allow(clippy::new_without_default)]
impl App {
    /// "toggle" table selection
    pub fn toggle_table_selection(&mut self) {
        if self.tier_list_state.selected_column().is_some() {
            self.tier_list_state.select_cell(None);
        } else {
            self.tier_list_state.select_cell(Some((6, 1))); // FIXME: hardcoded...
        }
    }

    pub fn new() -> Self {
        let items = std::fs::read_to_string("languages.txt")
            .unwrap()
            .lines()
            .map(|language_str| TierListItem {
                text: language_str.to_string(),
                area: None,
            })
            .collect();

        Self {
            current_screen: Screen::TierList,
            quitting: false,
            tier_list_state: TableState::new(),
            items,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.quitting {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let [header, body, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let footer_block = Block::new()
            .title_style(Style::default().bold())
            .title_alignment(HorizontalAlignment::Center)
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Color::LightBlue))
            .borders(Borders::TOP);
        frame.render_widget(footer_block.clone().title(format!(" {PKG_NAME} ")), header);
        frame.render_widget(footer_block, footer);

        match self.current_screen {
            Screen::StartMenu => {
                let welcome_text = Text::from(Line::from(vec![
                    "welcome to ".into(),
                    PKG_NAME.bold(),
                    ", a bloat-free, minimalist way to create tier lists".into(),
                ]));
                let welcome_paragraph = Paragraph::new(welcome_text)
                    .wrap(Wrap { trim: true })
                    .centered();

                frame.render_widget(welcome_paragraph, body);
            }
            Screen::TierList => {
                let tiers = [
                    "S".on_red(),
                    "A".into(),
                    "B".into(),
                    "C".into(),
                    "D".into(),
                    "F".into(),
                    "".into(),
                ];

                const NUM_COLUMNS: usize = 2;
                let col_constraints = [Constraint::Length(9); NUM_COLUMNS];
                let row_constraints = [Constraint::Fill(1); 7];

                let horizontal = Layout::horizontal(col_constraints).spacing(1);
                let vertical = Layout::vertical(row_constraints).spacing(1);

                let cells = body
                    .layout_vec(&vertical)
                    .into_iter()
                    .flat_map(|row| row.layout_vec(&horizontal));
                let last_cell = cells.clone().last(); // FIXME: !

                for (cell, letter) in cells.step_by(NUM_COLUMNS).zip(tiers) {
                    frame.render_widget(letter, cell);
                }

                for item in &self.items {
                    frame
                        .render_widget(item.text.as_str(), item.area.unwrap_or(last_cell.unwrap()));
                }
            }
            _ => todo!(),
        };
    }

    fn handle_events(&mut self) -> Result<()> {
        use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};

        let event = crossterm::event::read()?;

        match self.current_screen {
            Screen::TierList => match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.tier_list_state.select_previous();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.tier_list_state.select_next();
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.tier_list_state.select_next_column();
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        if let Some(column_idx) = self.tier_list_state.selected_column()
                            && column_idx > 1
                        {
                            self.tier_list_state.select_previous_column();
                        }
                    }
                    KeyCode::Esc => {
                        self.tier_list_state.select(None);
                    }
                    KeyCode::Char('q') => self.quit(),
                    _ => {}
                },
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        println!("clidked {}", mouse_event.row);
                    }
                    MouseEventKind::Moved => {
                        println!("{}", mouse_event.row);
                    }
                    _ => {
                        dbg!(mouse_event);
                        todo!();
                    }
                },
                _ => {}
            },
            _ => todo!(),
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.quitting = true;
    }
}
