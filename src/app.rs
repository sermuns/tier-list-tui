use color_eyre::Result;
use ratatui::{
    DefaultTerminal, crossterm,
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub enum Screen {
    StartMenu,
    TierList,
    UploadingImage,
}

pub struct App {
    pub current_screen: Screen,
    should_quit: bool,
}

#[allow(clippy::new_without_default)]
impl App {
    pub fn new() -> Self {
        Self {
            current_screen: Screen::StartMenu,
            should_quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        match self.current_screen {
            Screen::StartMenu => {
                let main_block = Block::new()
                    .title(format!(" {PKG_NAME} "))
                    .title_style(Style::default().bold())
                    .title_alignment(HorizontalAlignment::Center)
                    .borders(Borders::TOP | Borders::BOTTOM)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::LightBlue));

                let [header, body, footer] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .areas(frame.area());

                frame.render_widget(main_block, frame.area());

                let welcome_text = Text::from(Line::from(vec![
                    "welcome to ".into(),
                    PKG_NAME.bold(),
                    ", a bloat-free, minimalist way to create tier lists".into(),
                ]));
                let welcome_paragraph = Paragraph::new(welcome_text)
                    .wrap(Wrap { trim: true })
                    .block(Block::bordered())
                    .centered();

                frame.render_widget(
                    welcome_paragraph,
                    body.centered(Constraint::Max(80), Constraint::Max(30)),
                );
            }
            _ => todo!(),
        };
    }

    fn handle_events(&mut self) -> Result<()> {
        if crossterm::event::read()?.is_key_press() {
            self.should_quit = true;
        }
        Ok(())
    }
}
