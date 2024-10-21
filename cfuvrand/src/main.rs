use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Widget, Wrap},
    DefaultTerminal,
};
use std::io;

#[derive(Debug, Default)]
pub struct State {
    exit: bool,
}

const TILDES: border::Set = border::Set {
    top_left: " ",
    bottom_left: "~",
    vertical_left: "~",
    top_right: " ",
    bottom_right: " ",
    vertical_right: " ",
    horizontal_top: " ",
    horizontal_bottom: " ",
};

fn placeholder_paragraph() -> Paragraph<'static> {
    let text = Text::from(vec![
        Line::raw("CFUVRand — CFU Vernandskogo Randomizer"),
        Line::default(),
        Line::raw(format!("version {}", env!("CARGO_PKG_VERSION"))),
        Line::raw("CFUVRand is open source and freely distributable"),
    ]);
    Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

impl State {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| f.render_widget(&*self, f.area()))?;
            self.handlers()?;
        }
        Ok(())
    }
    fn handlers(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.exit = true;
            }
        }
        Ok(())
    }
}
impl Widget for &State {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Max(1)])
            .split(area);
        let text = placeholder_paragraph();
        let block = Block::bordered()
            .padding(Padding::vertical(layout[0].height / 2 - 4))
            .border_set(TILDES)
            .border_style(Style::new().blue());

        text.block(block).render(layout[0], buf);
        Paragraph::new("0,0-1     All")
            .alignment(Alignment::Right)
            .block(Block::new().padding(Padding::horizontal(2)))
            .render(layout[1], buf);
    }
}

fn main() -> io::Result<()> {
    let result = State::default().run(ratatui::init());
    ratatui::restore();
    result
}
