use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = cursive::default();

    siv.set_window_title("pAPIrus");
    siv.add_global_callback('q', |s| s.quit());
    siv.add_fullscreen_layer(
        Dialog::around(TextView::new("Hello World! (press 'q' to quit)"))
            .title("pAPIrus")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}
