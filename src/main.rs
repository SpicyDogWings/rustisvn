mod cursor;
mod files;
mod renders;
mod svn;
use crate::{
    cursor::{move_cursor_down, move_cursor_up},
    files::copy_file,
    renders::create_layout,
    svn::{SvnClient, SvnStatusList, style_for_status},
};
use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph},
};
use std::{
    fs::canonicalize,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(name = "Rustisvn")]
#[command(about = "A TUI for svn", long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    directory: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let terminal = ratatui::init();
    let proyect_path = canonicalize(&args.directory).unwrap();
    let result = App::new(proyect_path).run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    proyect_path: PathBuf,
    //svn: SvnClient,
    status_lines: SvnStatusList,
    selected: usize,
}

impl App {
    pub fn new<T: AsRef<Path>>(proyect_path: T) -> Self {
        let path = proyect_path.as_ref().to_path_buf();
        let svn = SvnClient::new(&path);
        let status_lines = svn.svn_status();
        Self {
            running: true,
            proyect_path: path.clone(),
            //svn: svn,
            status_lines,
            selected: 0,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = create_layout(&frame);
        let directory_path = Paragraph::new(self.proyect_path.to_string_lossy())
            .block(Block::bordered().title("Project directory"));
        let items: Vec<ListItem> = self
            .status_lines
            .entries()
            .iter()
            .enumerate()
            .map(|(_i, entry)| {
                let style = style_for_status(&entry.state());
                let line = Line::from(vec![
                    Span::styled(String::from(&entry.state().to_string()), style),
                    Span::raw(" "),
                    Span::raw(entry.file().to_string_lossy()),
                ]);
                ListItem::new(line)
            })
            .collect();
        let mut state = ListState::default().with_selected(Some(self.selected));
        let list = List::new(items)
            .block(
                Block::bordered()
                    .title(" Status ")
                    .title_top(Line::from(self.proyect_path.to_string_lossy()).right_aligned())
                    .border_style(Style::new().blue().bold())
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::new().bg(Color::DarkGray));
        let _selected_list = Block::bordered()
            .title(" Selected ")
            .border_style(Style::new().gray())
            .border_type(BorderType::Rounded);
        //frame.render_stateful_widget(list, status_block, &mut state);
        //frame.render_widget(selected_list, selected_block);
        frame.render_widget(directory_path, layout[0]);
        frame.render_stateful_widget(list, layout[1], &mut state);
        frame.render_widget(_selected_list, layout[2]);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                self.selected = move_cursor_up(self.selected);
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                self.selected = move_cursor_down(self.selected, self.status_lines.entries().len());
            }
            (_, KeyCode::Char('y')) => {
                let _ = copy_file(self.selected, &self.status_lines.entries())
                    .expect("Error al copiar el archivo");
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
