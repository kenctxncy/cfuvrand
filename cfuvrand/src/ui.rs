use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Widget, Wrap},
};

#[derive(Debug)]
pub enum Page<'a> {
    Text(Text<'a>),
    Example,
}
impl Default for Page<'_> {
    fn default() -> Self {
        Page::Text(intro_text())
    }
}
impl Widget for &Page<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Page::Text(txt) => {
                let block = Block::bordered()
                    .padding(vertical_center(txt, &area))
                    .border_set(TILDES_BORDER)
                    .border_style(Style::new().blue());
                Paragraph::new(txt.clone())
                    .centered()
                    .wrap(Wrap { trim: true })
                    .block(block)
                    .render(area, buf);
            }
            Page::Example => {
                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(Constraint::from_fills([1, 1]))
                    .split(area);
                let text = intro_text();
                text.clone().render(layout[0], buf);
                text.render(layout[1], buf);
            }
        }
    }
}

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

pub fn intro_text<'a>() -> Text<'a> {
    Text::from(vec![
        Line::raw("CFUVRand — CFU Vernandskogo Randomizer"),
        Line::default(),
        Line::raw(format!("version {}", env!("CARGO_PKG_VERSION"))),
        Line::raw("CFUVRand is open source and freely distributable"),
    ])
}

pub fn vertical_center(w: &Text, area: &Rect) -> Padding {
    Padding::vertical(area.height / 2 - w.height().try_into().unwrap_or(u16::MAX))
}
