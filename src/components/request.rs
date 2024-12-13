use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::style::{palette::tailwind, Stylize};
use ratatui::widgets::{Block, Padding, Paragraph, Tabs};
use ratatui::Frame;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::components::Component;
use crate::config::Config;

#[derive(Default, Display, FromRepr, EnumIter, Clone, Copy)]
pub enum SelectedTab {
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
pub struct Request {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    selected_tab: SelectedTab,
}

impl SelectedTab {
    pub fn previous(self) -> Self {
        let current_idx: usize = self as usize;
        let previous_idx = current_idx.saturating_sub(1);

        Self::from_repr(previous_idx).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let current_idx: usize = self as usize;
        let next_idx = current_idx.saturating_add(1);

        Self::from_repr(next_idx).unwrap_or(self)
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Query => tailwind::BLUE,
            Self::Body => tailwind::GREEN,
            Self::Headers => tailwind::YELLOW,
            Self::Auth => tailwind::RED,
        }
    }

    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c900)
            .bg(self.palette().c900)
            .into()
    }

    fn block(self) -> Block<'static> {
        Block::bordered().padding(Padding::horizontal(1))
    }

    fn render_tab_query(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Query")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab_body(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Body").block(self.block()).render(area, buf);
    }

    fn render_tab_headers(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Headers")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab_auth(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Auth").block(self.block()).render(area, buf);
    }
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Line::raw("◄ ► to change tab").centered().render(area, buf);
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_idx = self.selected_tab as usize;

        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_idx)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }
}

impl Widget for &mut Request {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        self.render_tabs(header_area, buf);
        self.selected_tab.render(inner_area, buf);
        self.render_footer(footer_area, buf);
    }
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::Query => self.render_tab_query(area, buf),
            Self::Body => self.render_tab_body(area, buf),
            Self::Headers => self.render_tab_headers(area, buf),
            Self::Auth => self.render_tab_auth(area, buf),
        }
    }
}

impl Component for Request {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key {
            KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Left,
                ..
            } => {
                self.previous_tab();
            }
            KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Right,
                ..
            } => {
                self.next_tab();
            }
            _ => {}
        }

        Ok(None)
    }

    #[allow(unused_variables)]
    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
        // TODO: allow clicking to change tabs
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(self, area);

        Ok(())
    }
}
