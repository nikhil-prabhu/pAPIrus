pub(crate) mod app;
pub(crate) mod input;
pub(crate) mod request;
pub(crate) mod response;

use cursive::theme::{BaseColor, Color, Style};
use cursive::{Printer, Vec2, View};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
use syntect::parsing::SyntaxSet;

pub(crate) struct CodeView {
    code_lines: Vec<(String, Vec<(SyntectStyle, String)>)>,
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
