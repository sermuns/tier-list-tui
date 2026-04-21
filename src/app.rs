use image::DynamicImage;
use ratatui::{
    DefaultTerminal, crossterm,
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};
use ratatui_image::picker::Picker;
use std::{cmp::min, path::PathBuf, time::Duration};

const TIERS: [&str; 7] = ["S", "A", "B", "C", "D", "F", " "];
const NUM_TIERS: usize = TIERS.len();

pub enum Screen {
    StartMenu,
    TierList,
    UploadingImage,
}

#[derive(Default)]
pub struct Item {
    image: Option<DynamicImage>,
}

#[derive(Default)]
pub struct Tier<'a> {
    letter_span: Span<'a>,
    items: Vec<Item>,
}

pub struct App<'a> {
    pub current_screen: Screen,
    running: bool,
    tiers: [Tier<'a>; NUM_TIERS],

    /// row_idx, item_idx
    focus: (usize, usize),

    grabbed: Option<Item>,

    picker: Picker,
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

        // let items = &mut self.tiers[row_idx].items;
        // if let Some(grabbed) = &mut self.grabbed {
        //     if col_idx < items.len() {
        //         items.insert(col_idx, grabbed.clone());
        //     } else {
        //         items.push(grabbed.clone());
        //     }
        //     self.grabbed = None;
        // } else {
        //     self.grabbed = Some(items.remove(col_idx));
        // }
    }

    pub fn new(images_path: PathBuf) -> color_eyre::Result<Self> {
        let mut tiers = TIERS.map(|letter_span| Tier {
            letter_span: letter_span.into(),
            ..Default::default()
        });
        tiers[0].letter_span = TIERS[0].red(); // shitty

        // TODO:
        tiers[NUM_TIERS - 1].items = std::fs::read_dir(images_path)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                path.is_file().then_some(Item {
                    image: image::ImageReader::open(path).ok()?.decode().ok(),
                })
            })
            .collect();

        Ok(Self {
            tiers,
            current_screen: Screen::TierList,
            running: true,
            focus: (NUM_TIERS - 1, 0),
            grabbed: None,
            picker: Picker::from_query_stdio()?,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&*self, frame.area()))?;
            self.handle_crossterm_event()?;
        }
        Ok(())
    }

    fn handle_crossterm_event(&mut self) -> color_eyre::Result<()> {
        use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};

        if !crossterm::event::poll(Duration::from_secs(1))? {
            return Ok(());
        }

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
        self.running = false;
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_block = Block::bordered()
            .title_style(Style::default().bold())
            .title(concat!(" ", env!("CARGO_PKG_NAME"), " "))
            .title_alignment(HorizontalAlignment::Center);

        (&title_block).render(area, buf);

        let area = title_block.inner(area);

        match self.current_screen {
            Screen::StartMenu => {
                let welcome_text = Line::from(vec![
                    "welcome to ".into(),
                    env!("CARGO_PKG_NAME").bold(),
                    ", a bloat-free, minimalist way to create tier lists".into(),
                ]);
                Paragraph::new(welcome_text)
                    .wrap(Wrap { trim: true })
                    .centered()
                    .render(area, buf);
            }
            Screen::TierList => {
                let tier_rows: [_; NUM_TIERS] =
                    area.layout(&Layout::vertical([Constraint::Fill(1); NUM_TIERS]));

                let max_items_in_row = self
                    .tiers
                    .iter()
                    .map(|tier| tier.items.len())
                    .max()
                    .unwrap_or(0);

                for ((row_index, row_area), tier) in tier_rows.iter().enumerate().zip(&self.tiers) {
                    let [letter_span_area, outer_items_area] =
                        row_area.layout(&Layout::horizontal([
                            Constraint::Length(1),
                            Constraint::Fill(1),
                        ]));

                    (&tier.letter_span).render(letter_span_area, buf);

                    let item_areas =
                        Layout::horizontal(vec![Constraint::Fill(1); max_items_in_row])
                            .split(outer_items_area);

                    for ((item_index, item), item_area) in
                        tier.items.iter().enumerate().zip(item_areas.iter())
                    {
                        if let Some(dyn_img) = &item.image {
                            ratatui_image::Image::new(
                                &self
                                    .picker
                                    .new_protocol(
                                        dyn_img.clone(), // FIXME:
                                        *item_area,
                                        ratatui_image::Resize::Fit(None),
                                    )
                                    .unwrap(),
                            )
                            .render(*item_area, buf);
                        }
                        // if self.focus == (row_index, item_index) {
                        //     if self.grabbed.is_some() {
                        //         frame.render_widget(
                        //             item.label.clone().black().on_yellow(),
                        //             *item_are,
                        //         );
                        //     } else {
                        //         frame.render_widget(item.label.clone().on_dark_gray(), *item_are);
                        //     }
                        // } else {
                        //     frame.render_widget(&item.label, *item_are);
                        // }
                    }
                }
            }
            _ => todo!(),
        };
    }
}
