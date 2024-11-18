pub(crate) mod types;
pub(crate) mod views;

use cursive::style::BaseColor;
use cursive::theme::{BorderStyle, Theme};
use cursive::theme::{Color, Palette, PaletteColor};
use cursive::traits::Resizable;

use crate::views::app::app_view;

fn main() {
    let mut siv = cursive::default();

    siv.set_theme(default_theme());
    siv.set_window_title("pAPIrus");
    siv.add_fullscreen_layer(app_view().full_screen());

    siv.run();
}

fn default_theme() -> Theme {
    let mut palette = Palette::default();

    palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Primary] = Color::Light(BaseColor::White);
    palette[PaletteColor::Secondary] = Color::Dark(BaseColor::White);
    palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Highlight] = Color::Light(BaseColor::Green);
    palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::Green);

    Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette,
    }
}
