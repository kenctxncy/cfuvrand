use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};
use std::io;
use tui_textarea::{Input, Key, TextArea};

#[derive(Default, Debug, PartialEq)]
enum SelectedPane {
    #[default]
    None,
    CommandInput,
}

#[derive(Debug, Default)]
struct Model<'a> {
    input_area: TextArea<'a>,
    selected_pane: SelectedPane,
    exit: bool,
}

const EMPTY_BORDER: border::Set = border::Set {
    top_left: " ",
    bottom_left: " ",
    vertical_left: " ",
    top_right: " ",
    bottom_right: " ",
    vertical_right: " ",
    horizontal_top: " ",
    horizontal_bottom: " ",
};

const TILDES_BORDER: border::Set = border::Set {
    bottom_left: "~",
    vertical_left: "~",
    ..EMPTY_BORDER
};
const COLON_BORDER: border::Set = border::Set {
    top_left: ":",
    bottom_left: ":",
    vertical_left: ":",
    ..EMPTY_BORDER
};

fn intro_paragraph<'a>() -> Paragraph<'a> {
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

fn cmd_placeholder_paragraph<'a>() -> Paragraph<'a> {
    Paragraph::new("0,0-1     All")
        .alignment(Alignment::Right)
        .block(Block::new().padding(Padding::horizontal(2)))
}

impl Model<'_> {
    fn new() -> Self {
        let mut m = Self::default();
        m.input_area
            .set_block(Block::new().borders(Borders::LEFT).border_set(COLON_BORDER));
        m.input_area.set_cursor_line_style(Style::default());
        m
    }
    fn run<W>(&mut self, terminal: &mut Terminal<CrosstermBackend<W>>) -> io::Result<()>
    where
        W: io::Write,
    {
        while !self.exit {
            terminal.draw(|f| f.render_widget(&*self, f.area()))?;
            self.handlers()?;
        }
        Ok(())
    }
    fn handlers(&mut self) -> io::Result<()> {
        match self.selected_pane {
            SelectedPane::CommandInput => match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    self.selected_pane = SelectedPane::None;
                }
                Input {
                    key: Key::Char('m'),
                    ctrl: true,
                    ..
                } => {}

                Input {
                    key: Key::Enter, ..
                } => {
                    let cmd = self.input_area.lines()[0].trim();
                    match cmd {
                        "help" => panic!("Помощи нет"),
                        _ if cmd.starts_with("q") => {
                            self.exit = true;
                        }
                        _ => {}
                    }
                }
                input => {
                    self.input_area.input(input);
                }
            },
            SelectedPane::None => {
                if let Input {
                    key: Key::Char(':'),
                    ..
                } = crossterm::event::read()?.into()
                {
                    self.selected_pane = SelectedPane::CommandInput;
                }
            }
        }
        Ok(())
    }
}

impl Widget for &Model<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Max(1)])
            .split(area);
        let text = intro_paragraph();
        let block = Block::bordered()
            .padding(Padding::vertical(layout[0].height / 2 - 4))
            .border_set(TILDES_BORDER)
            .border_style(Style::new().blue());

        text.block(block).render(layout[0], buf);
        match self.selected_pane {
            SelectedPane::CommandInput => self.input_area.render(layout[1], buf),
            _ => cmd_placeholder_paragraph().render(layout[1], buf),
        }
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    Model::new().run(&mut term)?;

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    Ok(())
}
