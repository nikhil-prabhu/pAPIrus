pub(crate) mod types;
pub(crate) mod views;

use cursive::traits::Resizable;

use crate::views::app::app_view;

fn main() {
    let mut siv = cursive::default();

    siv.load_toml(include_str!("themes/base16-default-dark.toml"))
        .unwrap();
    siv.set_window_title("pAPIrus");
    siv.add_fullscreen_layer(app_view().full_screen());

    siv.run();
}
