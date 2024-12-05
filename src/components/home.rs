use color_eyre::Result;
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config, PKG_NAME};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
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

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            Action::Mouse(MouseEvent {
                kind: MouseEventKind::Down(_),
                ..
            }) => {
                // add any logic here that should run on every mouse click (down)
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
            Constraint::Min(1),
            Constraint::Min(3),
            Constraint::Percentage(100),
        ])
        .split(area);
        let title_area = main_area[0];
        let _url_area = main_area[1];
        let [_req_area, _resp_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area[2]);

        frame.render_widget(title, title_area);
        Ok(())
    }
}
