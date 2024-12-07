use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};
use ratatui::widgets::{Block, Borders};
use ratatui::prelude::*;
use ratatui::style::Styled;
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;
use super::Component;
use crate::{action::Action, config::Config, PKG_NAME};

/// The current area of focus.
#[derive(Default, Copy, Clone, PartialEq)]
enum Focus {
    Url,
    #[default]
    Home,
}

/// A clickable area on the screen.
#[derive(Default)]
struct Clickable {
    /// The rectangular area of the clickable.
    rect: Rect,
    /// The focus ID that should be set when the clickable is clicked.
    focus: Focus,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    focus: Focus,
    clickable: Vec<Clickable>,
    url_input: TextArea<'static>,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    fn render_url_input(&mut self, frame: &mut Frame, area: Rect) {
        self.clickable.push(Clickable {
            rect: area,
            focus: Focus::Url,
        });

        self.url_input.set_placeholder_text("Enter a URL...");
        self.url_input.set_cursor_line_style(Style::default().fg(Color::White));

        let info = "Press <Enter> to send request";
        if self.focus == Focus::Url {
            self.url_input.set_block(Block::default().borders(Borders::ALL).set_style(Color::White).title(info));
        } else {
            self.url_input.set_block(Block::default().borders(Borders::ALL).set_style(Color::DarkGray).title(info));
        }

        frame.render_widget(&self.url_input, area);
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match self.focus {
            Focus::Url => {
                // FIXME: Discard global key events when the URL input is focused
                // FIXME: Disallow multiple lines in the URL input
                self.url_input.input(key);
                Ok(None)
            }
            _ => Ok(None)
        }
    }

    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
        match mouse {
            MouseEvent {
                kind: MouseEventKind::Down(_),
                column,
                row,
                ..
            } => {
                for c in &self.clickable {
                    self.focus = Focus::Home;

                    if c.rect.contains(Position { x: column, y: row }) {
                        self.focus = c.focus;
                    }
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let title = Block::new()
            .borders(Borders::TOP)
            .title(format!(" {PKG_NAME} "))
            .title_alignment(Alignment::Center);

        let main_area = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Percentage(100),
        ]).split(area);
        let title_area = main_area[0];
        let url_area = main_area[1];
        let [_req_area, _resp_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area[2]);

        frame.render_widget(title, title_area);
        self.render_url_input(frame, url_area);
        Ok(())
    }
}
