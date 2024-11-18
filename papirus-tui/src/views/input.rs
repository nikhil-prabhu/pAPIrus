use cursive::align::HAlign;
use cursive::traits::{Nameable, Resizable};
use cursive::view::Margins;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};

use crate::types::HttpMethod;

pub(crate) fn input_view() -> Dialog {
    let mut layout = LinearLayout::horizontal();

    let mut method_select = SelectView::new().h_align(HAlign::Center).popup();
    // TODO: add colors for method labels
    method_select.add_item("GET", HttpMethod::GET);
    method_select.add_item("POST", HttpMethod::POST);
    method_select.add_item("PUT", HttpMethod::PUT);
    method_select.add_item("DELETE", HttpMethod::DELETE);
    method_select.add_item("PATCH", HttpMethod::PATCH);
    method_select.add_item("OPTIONS", HttpMethod::OPTIONS);
    method_select.add_item("HEAD", HttpMethod::HEAD);

    let url_input = EditView::new();

    let send_button = Button::new("Send", |_| {});

    layout.add_child(
        method_select
            .fixed_width(10)
            .with_name("http_method_select"),
    );
    layout.add_child(DummyView::new().fixed_width(2));
    layout.add_child(url_input.full_width().with_name("request_url_input"));
    layout.add_child(DummyView::new().fixed_width(2));
    layout.add_child(send_button.fixed_width(10).with_name("send_request_button"));

    Dialog::around(layout)
        .title("Press <Enter> to send request")
        .padding(Margins::lrtb(1, 1, 1, 1))
}
