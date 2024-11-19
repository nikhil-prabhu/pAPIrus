use cursive::traits::Nameable;
use cursive::views::DummyView;
use cursive_tabs::TabPanel;

pub(crate) fn request_view() -> TabPanel {
    let body_input = DummyView::new();
    let query_input = DummyView::new();
    let headers_input = DummyView::new();
    let auth_input = DummyView::new();

    let mut request_panel = TabPanel::new()
        .with_tab(query_input.with_name("Query"))
        .with_tab(body_input.with_name("Body"))
        .with_tab(headers_input.with_name("Headers"))
        .with_tab(auth_input.with_name("Auth"));
    request_panel.set_active_tab("Query").unwrap();

    request_panel
}
