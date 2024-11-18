use cursive::traits::Resizable;
use cursive::views::{DummyView, LinearLayout};

use crate::views::input::input_view;

pub(crate) fn app_view() -> LinearLayout {
    let mut layout = LinearLayout::vertical();

    layout.add_child(DummyView::new().fixed_height(1));
    layout.add_child(input_view());

    layout
}
