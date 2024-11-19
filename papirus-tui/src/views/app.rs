use cursive::traits::Resizable;
use cursive::views::{DummyView, LinearLayout};

use crate::views::input::input_view;
use crate::views::request::request_view;
use crate::views::response::response_view;

pub(crate) fn app_view() -> LinearLayout {
    let mut input_area = LinearLayout::vertical();
    let mut output_area = LinearLayout::horizontal();

    output_area.add_child(request_view().full_height().full_width());
    output_area.add_child(response_view().full_height().full_width());

    input_area.add_child(DummyView::new().fixed_height(1));
    input_area.add_child(input_view());
    input_area.add_child(DummyView::new().fixed_height(1));
    input_area.add_child(output_area);

    input_area
}
