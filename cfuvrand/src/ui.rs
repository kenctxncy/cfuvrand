use ratatui::{
    layout::Rect,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Wrap},
};

pub const EMPTY_BORDER: border::Set = border::Set {
    top_left: " ",
    bottom_left: " ",
    vertical_left: " ",
    top_right: " ",
    bottom_right: " ",
    vertical_right: " ",
    horizontal_top: " ",
    horizontal_bottom: " ",
};

pub const TILDES_BORDER: border::Set = border::Set {
    bottom_left: "~",
    vertical_left: "~",
    ..EMPTY_BORDER
};

pub const COLON_BORDER: border::Set = border::Set {
    top_left: ":",
    bottom_left: ":",
    vertical_left: ":",
    ..EMPTY_BORDER
};

pub fn intro_paragraph<'a>() -> Paragraph<'a> {
    let text = Text::from(vec![
        Line::raw("CFUVRand — CFU Vernandskogo Randomizer"),
        Line::default(),
        Line::raw(format!("version {}", env!("CARGO_PKG_VERSION"))),
        Line::raw("CFUVRand is open source and freely distributable"),
    ]);
    Paragraph::new(text).centered().wrap(Wrap { trim: true })
}

pub fn cmd_placeholder_paragraph<'a>() -> Paragraph<'a> {
    Paragraph::new("0,0-1     All")
        .right_aligned()
        .block(Block::new().padding(Padding::horizontal(2)))
}

pub fn vertical_center(w: &ratatui::widgets::Paragraph, area: &Rect) -> Padding {
    Padding::vertical(area.height / 2 - w.line_count(area.width).try_into().unwrap_or(u16::MAX))
}
