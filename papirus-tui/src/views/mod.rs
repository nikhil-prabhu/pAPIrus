pub(crate) mod app;
pub(crate) mod input;
pub(crate) mod request;
pub(crate) mod response;

use cursive::theme::{BaseColor, Color, Style};
use cursive::traits::Resizable;
use cursive::views::{LinearLayout, TextView};
use cursive::{Printer, Vec2, View};
use std::time::Duration;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
use syntect::parsing::SyntaxSet;

use crate::utils::format_duration;

pub(crate) struct ResponseInfoView {
    layout: LinearLayout,
}

pub(crate) struct CodeView {
    code_lines: Vec<(String, Vec<(SyntectStyle, String)>)>,
}

impl ResponseInfoView {
    pub(crate) fn new(status_code: &str, duration: Duration, response_size: usize) -> Self {
        let mut layout = LinearLayout::horizontal();

        // Create text views for each piece of information
        let status_view = TextView::new(format!("Status: {}", status_code)).full_width();

        let duration_view =
            TextView::new(format!("Time: {}", format_duration(duration))).full_width();

        let size_view = TextView::new(format!("Size: {} B", response_size)).full_width();

        // Add components to the horizontal layout
        layout.add_child(status_view);
        layout.add_child(duration_view);
        layout.add_child(size_view);

        Self { layout }
    }
}

impl CodeView {
    pub(crate) fn new(code: &str, language: &str) -> Self {
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ss
            .find_syntax_by_extension(language)
            .unwrap_or_else(|| ss.find_syntax_plain_text());
        let theme = &ts.themes["base16-ocean.dark"];

        let mut highlighter = HighlightLines::new(syntax, theme);

        let code_lines = code
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let ranges = highlighter
                    .highlight_line(line, &ss)
                    .unwrap()
                    .into_iter()
                    .map(|(style, text)| (style, text.to_string()))
                    .collect();

                (format!("{:4} ", i + 1), ranges)
            })
            .collect();

        Self { code_lines }
    }
}

impl View for ResponseInfoView {
    fn draw(&self, printer: &Printer) {
        self.layout.draw(printer);
    }

    fn layout(&mut self, size: Vec2) {
        self.layout.layout(size);
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.layout.required_size(constraint)
    }
}

impl View for CodeView {
    fn draw(&self, printer: &Printer) {
        for (i, (line_number, highlighted)) in self.code_lines.iter().enumerate() {
            printer.with_style(Style::from(Color::Dark(BaseColor::Cyan)), |p| {
                p.print((0, i), line_number);
            });

            let mut x_offset = line_number.len();
            for &(style, ref text) in highlighted {
                let fg_color =
                    Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);

                printer.with_style(Style::from(fg_color), |p| {
                    p.print((x_offset, i), text.as_str());
                });

                x_offset += text.len();
            }
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        let max_line_length = self
            .code_lines
            .iter()
            .map(|(line_number, highlighted)| {
                line_number.len()
                    + highlighted
                        .iter()
                        .map(|(_, text)| text.len())
                        .sum::<usize>()
            })
            .max()
            .unwrap_or(0);

        Vec2::new(max_line_length, self.code_lines.len())
    }
}
