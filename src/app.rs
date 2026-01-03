use color_eyre::Result;
use ratatui::{
    DefaultTerminal, crossterm,
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};
use std::cmp::min;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const NUM_TIERS: usize = 7; // FIXME: ...

pub enum Screen {
    StartMenu,
    TierList,
    UploadingImage,
}

#[derive(Default, Clone, Debug)]
pub struct Item<'a> {
    label: Span<'a>,
}

#[derive(Default)]
pub struct Tier<'a> {
    letter_span: Span<'a>,
    items: Vec<Item<'a>>,
}

pub struct App<'a> {
    pub current_screen: Screen,
    quitting: bool,
    tiers: [Tier<'a>; NUM_TIERS],

    /// row_idx, item_idx
    focus: (usize, usize),

    grabbed: Option<Item<'a>>,
}

enum InputMovement {
    Up,
    Down,
    Left,
    Right,
    Top,
    Bottom,
}

#[allow(clippy::new_without_default)]
impl App<'_> {
    fn move_focus(&mut self, movement: InputMovement) {
        let (row_idx, col_idx) = &mut self.focus;

        match movement {
            InputMovement::Up => {
                *row_idx = row_idx.saturating_sub(1);
            }
            InputMovement::Down => {
                if *row_idx >= NUM_TIERS - 1 {
                    return;
                }
                *row_idx += 1;
            }
            InputMovement::Top => {
                *row_idx = 0;
            }
            InputMovement::Left => {
                *col_idx = col_idx.saturating_sub(1);
            }
            InputMovement::Right => {
                *col_idx += 1;
            }
            InputMovement::Bottom => {
                *row_idx = NUM_TIERS - 1; // FIXME:
            }
        }
        let last_idx_in_row = self.tiers[*row_idx].items.len().saturating_sub(1);
        *col_idx = min(*col_idx, last_idx_in_row);
    }

    /// if something grabbed, place at focus
    /// if nothing grabbed, grab focused.
    fn grab_or_place(&mut self) {
        let (row_idx, col_idx) = self.focus;

        let items = &mut self.tiers[row_idx].items;
        if let Some(grabbed) = &mut self.grabbed {
            if col_idx < items.len() {
                items.insert(col_idx, grabbed.clone());
            } else {
                items.push(grabbed.clone());
            }
            self.grabbed = None;
        } else {
            self.grabbed = Some(items.remove(col_idx));
        }
    }

    pub fn new() -> Self {
        let mut tiers = [
            "S".bold().red(),
            "A".into(),
            "B".into(),
            "C".into(),
            "D".into(),
            "F".into(),
            " ".into(),
        ]
        .map(|letter_span| Tier {
            letter_span,
            ..Default::default()
        });

        // TODO:
        tiers[NUM_TIERS - 1].items = std::fs::read_to_string("languages.txt")
            .unwrap()
            .lines()
            .map(|line| Item {
                label: line.to_owned().into(),
            })
            .collect();

        Self {
            tiers,
            current_screen: Screen::TierList,
            quitting: false,
            focus: (NUM_TIERS - 1, 0),
            grabbed: None,
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

        let title_block = Block::new()
            .title_style(Style::default().bold())
            .title(format!(" {PKG_NAME} "))
            .title_alignment(HorizontalAlignment::Center)
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Color::LightBlue))
            .borders(Borders::TOP);
        frame.render_widget(
            title_block
                .clone()
                .title(format!("{:?} | {:?}", self.focus, self.grabbed)),
            footer,
        );
        frame.render_widget(title_block, header);

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
                let rows = Layout::vertical([Constraint::Fill(1); NUM_TIERS]).split(body);

                for ((row_idx, row_outer_area), tier) in rows.iter().enumerate().zip(&self.tiers) {
                    let mut items = tier.items.clone();
                    if let Some(grabbed) = &self.grabbed
                        && self.focus.0 == row_idx
                    {
                        if self.focus.1 <= items.len() {
                            items.insert(self.focus.1, grabbed.clone());
                        } else {
                            items.push(grabbed.clone());
                        }
                    }
                    let areas = Layout::horizontal(
                        [Constraint::Fill(1)]
                            .iter()
                            .chain(vec![Constraint::Length(20); items.len()].iter()),
                    )
                    // .flex(Flex::Start)
                    .split(*row_outer_area);
                    frame.render_widget(&tier.letter_span, areas[0]);

                    for ((item_idx, item), area) in
                        items.iter().enumerate().zip(areas.iter().skip(1))
                    {
                        if self.focus == (row_idx, item_idx) {
                            if self.grabbed.is_some() {
                                frame.render_widget(item.label.clone().black().on_yellow(), *area);
                            } else {
                                frame.render_widget(item.label.clone().on_dark_gray(), *area);
                            }
                        } else {
                            frame.render_widget(&item.label, *area);
                        }
                    }
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
                        self.move_focus(InputMovement::Up);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.move_focus(InputMovement::Down);
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.move_focus(InputMovement::Right);
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.move_focus(InputMovement::Left);
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        self.grab_or_place();
                    }
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::End | KeyCode::Char('G') => self.move_focus(InputMovement::Bottom),
                    KeyCode::Home | KeyCode::Char('g') => self.move_focus(InputMovement::Top),
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
