use cursive::traits::Nameable;
use cursive::views::DummyView;
use cursive_tabs::TabPanel;

pub(crate) fn response_view() -> TabPanel {
    let response_output = DummyView::new();
    let headers_output = DummyView::new();
    let timeline_output = DummyView::new();

    let mut response_panel = TabPanel::new()
        .with_tab(response_output.with_name("Response"))
        .with_tab(headers_output.with_name("Headers"))
        .with_tab(timeline_output.with_name("Timeline"));
    response_panel.set_active_tab("Response").unwrap();

    response_panel
}
