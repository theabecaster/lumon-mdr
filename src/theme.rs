use ratatui::style::{Color, Style};
use std::env;

#[derive(Clone, Copy)]
pub enum Palette {
    True,
    X256,
    Ansi,
}

pub fn detect() -> Palette {
    let colorterm = env::var("COLORTERM").unwrap_or_default().to_lowercase();
    if colorterm.contains("truecolor") {
        return Palette::True;
    }
    let term = env::var("TERM").unwrap_or_default();
    if term.contains("256") {
        return Palette::X256;
    } else {
        return Palette::Ansi;
    }
}

impl Palette {
    pub fn bg_style(self) -> Style {
        let navy = match self {
            Palette::True => Color::Rgb(18, 29, 56),
            Palette::X256 => Color::Indexed(17),
            Palette::Ansi => Color::Blue,
        };
        Style::default().bg(navy)
    }

    pub fn fg_style(self) -> Style {
        let fg_color = match self {
            Palette::True => Color::Rgb(88, 122, 148),
            Palette::X256 => Color::Indexed(66),
            Palette::Ansi => Color::Cyan,
        };
        Style::default().fg(fg_color)
    }
}
