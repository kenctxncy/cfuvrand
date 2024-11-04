use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

use std::io;
use tui_textarea::{Input, Key, TextArea};

mod ui;

#[derive(Debug, Default)]
struct Model<'a> {
    command_mode: bool,
    command_input: TextArea<'a>,
    page: ui::Page<'a>,
    exit: bool,
}

impl Model<'_> {
    fn new() -> Self {
        let mut m = Self::default();
        m.command_input.set_block(
            Block::new()
                .borders(Borders::LEFT)
                .border_set(ui::COLON_BORDER),
        );
        m.command_input.set_cursor_line_style(Style::default());
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
        if self.command_mode {
            match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    self.command_mode = false;
                }
                Input {
                    key: Key::Char('m'),
                    ctrl: true,
                    ..
                } => {}

                Input {
                    key: Key::Enter, ..
                } => {
                    let cmd = self.command_input.lines()[0].trim();
                    match cmd {
                        "help" => panic!("Помощи нет"),
                        _ if cmd.starts_with("q") => {
                            self.exit = true;
                        }
                        "example" => self.page = ui::Page::Example,

                        "prostite_please" => self.page = ui::Page::ForgiveMePlease,
                        _ => {}
                    }
                    self.command_input.delete_line_by_end();
                    self.command_input.delete_line_by_head();
                    self.command_mode = false;
                }
                input => {
                    self.command_input.input(input);
                }
            }
        } else if let Input {
            key: Key::Char(':'),
            ..
        } = crossterm::event::read()?.into()
        {
            self.command_mode = true;
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
        self.page.render(layout[0], buf);

        if self.command_mode {
            self.command_input.render(layout[1], buf);
        } else {
            Paragraph::new("0,0-1     All")
                .right_aligned()
                .block(Block::new().padding(Padding::horizontal(2)))
                .render(layout[1], buf);
        }
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut term = ratatui::init();

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
