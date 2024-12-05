use color_eyre::Result;
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;
use tui_big_text::{BigTextBuilder, PixelSize};

use super::Component;
use crate::{action::Action, config::Config};

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
            Action::Mouse(MouseEvent { kind: MouseEventKind::Down(_), .. }) => {
                // add any logic here that should run on every mouse click (down)
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let title = BigTextBuilder::default()
            .pixel_size(PixelSize::Full)
            .lines(["papirus".into()])
            .build();

        frame.render_widget(title, area);
        Ok(())
    }
}
