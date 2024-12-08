use std::collections::HashMap;

use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};
use ratatui::prelude::*;
use ratatui::style::{palette::tailwind, Styled};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Tabs};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

use super::Component;
use crate::app::Mode;
use crate::{action::Action, config::Config, PKG_NAME};

#[derive(Default, Display, FromRepr, EnumIter, Clone, Copy)]
enum RequestTab {
    #[default]
    #[strum(to_string = "Query")]
    Query,
    #[strum(to_string = "Body")]
    Body,
    #[strum(to_string = "Headers")]
    Headers,
    #[strum(to_string = "Auth")]
    Auth,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    mode: Mode,
    clickable: HashMap<Mode, Rect>,
    url_input: TextArea<'static>,
    selected_req_tab: RequestTab,
}

impl RequestTab {
    #[allow(dead_code)]
    fn previous(self) -> Self {
        let current_idx: usize = self as usize;
        let previous_idx = current_idx.saturating_sub(1);

        Self::from_repr(previous_idx).unwrap_or(self)
    }

    #[allow(dead_code)]
    fn next(self) -> Self {
        let current_idx: usize = self as usize;
        let next_idx = current_idx.saturating_add(1);

        Self::from_repr(next_idx).unwrap_or(self)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Query => tailwind::BLUE,
            Self::Body => tailwind::GREEN,
            Self::Headers => tailwind::YELLOW,
            Self::Auth => tailwind::RED,
        }
    }

    fn block(self) -> Block<'static> {
        Block::bordered().border_set(symbols::border::PROPORTIONAL_TALL).padding(Padding::horizontal(1)).border_style(self.palette().c700)
    }

    fn title(self) -> Line<'static> {
        format!("  {self}  ").fg(tailwind::SLATE.c200).bg(self.palette().c900).into()
    }

    fn render_tab_query(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Query").block(self.block()).render(area, buf);
    }

    fn render_tab_body(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Body").block(self.block()).render(area, buf);
    }

    fn render_tab_headers(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Headers").block(self.block()).render(area, buf);
    }

    fn render_tab_auth(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Auth").block(self.block()).render(area, buf);
    }
}

impl Widget for RequestTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::Query => self.render_tab_query(area, buf),
            Self::Body => self.render_tab_body(area, buf),
            Self::Headers => self.render_tab_headers(area, buf),
            Self::Auth => self.render_tab_auth(area, buf),
        }
    }
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    fn render_url_input(&mut self, frame: &mut Frame, area: Rect) {
        self.clickable.insert(Mode::Url, area);

        self.url_input.set_placeholder_text("Enter a URL...");
        self.url_input.set_cursor_line_style(Style::default().fg(Color::White));

        let info = "Press <Enter> to send request";
        if self.mode == Mode::Url {
            self.url_input.set_block(Block::default().borders(Borders::ALL).set_style(Color::White).title(info));
        } else {
            self.url_input.set_block(Block::default().borders(Borders::ALL).set_style(Color::DarkGray).title(info));
        }

        frame.render_widget(&self.url_input, area);
    }

    fn render_tabs(&mut self, frame: &mut Frame, area: Rect) {
        self.clickable.insert(Mode::Request, area);

        let mut block = Block::bordered().set_style(Color::DarkGray);
        if self.mode == Mode::Request {
            block = Block::bordered().set_style(Color::White);
        }

        let tabs = RequestTab::iter().map(|tab| tab.title()).collect::<Vec<_>>();
        let tabs = Tabs::new(tabs).highlight_style((Color::default(), self.selected_req_tab.palette().c900)).block(block);

        frame.render_widget(tabs, area);
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
        ]).split(area);
        let title_area = main_area[0];
        let url_area = main_area[1];
        let [req_area, _resp_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area[2]);

        frame.render_widget(title, title_area);
        self.render_url_input(frame, url_area);
        self.render_tabs(frame, req_area);
        Ok(())
    }
}
