use std::collections::HashMap;

use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};
use ratatui::prelude::*;
use ratatui::style::Styled;
use ratatui::widgets::{Block, Borders};
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

use super::Component;
use crate::app::Mode;
use crate::components::request::Request;
use crate::{action::Action, config::Config, PKG_NAME};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    mode: Mode,
    clickable: HashMap<Mode, Rect>,
    url_input: TextArea<'static>,
    request: Request,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    fn render_url_input(&mut self, frame: &mut Frame, area: Rect) {
        self.clickable.insert(Mode::Url, area);

        self.url_input.set_placeholder_text("Enter a URL...");
        self.url_input
            .set_cursor_line_style(Style::default().fg(Color::White));

        let info = "Press <Enter> to send request";
        if self.mode == Mode::Url {
            self.url_input.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .set_style(Color::White)
                    .title(info),
            );
        } else {
            self.url_input.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .set_style(Color::DarkGray)
                    .title(info),
            );
        }

        frame.render_widget(&self.url_input, area);
    }

    fn render_tabs(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.clickable.insert(Mode::Request, area);

        self.request.draw(frame, area)
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
        match self.mode {
            Mode::Url => {
                // FIXME: Discard global key events when the URL input is focused
                // FIXME: Disallow multiple lines in the URL input
                self.url_input.input(key);
                Ok(None)
            }
            Mode::Request => self.request.handle_key_event(key),
            _ => Ok(None),
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
                for (mode, rect) in self.clickable.iter() {
                    if rect.contains(Position { x: column, y: row }) {
                        self.mode = *mode;
                        break;
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
        ])
        .split(area);
        let title_area = main_area[0];
        let url_area = main_area[1];
        let [req_area, _resp_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area[2]);

        frame.render_widget(title, title_area);
        self.render_url_input(frame, url_area);
        self.render_tabs(frame, req_area)?;

        Ok(())
    }
}
