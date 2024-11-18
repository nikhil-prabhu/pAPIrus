pub(crate) mod types;
pub(crate) mod views;

use crate::views::app::app_view;
use cursive::traits::Resizable;
use cursive::views::Dialog;

fn main() {
    let mut siv = cursive::default();

    siv.set_window_title("pAPIrus");
    siv.add_fullscreen_layer(Dialog::around(app_view()).title("pAPIrus").full_screen());

    siv.run();
}
