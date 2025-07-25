mod cursor;
mod files;
mod renders;
mod svn;
use crate::{
    cursor::{move_cursor_down, move_cursor_up},
    files::copy_file,
    renders::{
        ProjectInfo, create_layout, create_section_commit, create_section_info,
        create_section_status, create_selected_items,
    },
    svn::{SvnClient, SvnStatusList, push_basic_commit},
};
use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, widgets::ListState};
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

#[derive(Debug, Default, PartialEq)]
pub enum AppMode {
    #[default]
    Normal,
    Commit,
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    proyect_path: PathBuf,
    svn: SvnClient,
    status_lines: SvnStatusList,
    selected: usize,
    mode: AppMode,
    commit_message: String,
}

impl App {
    pub fn new<T: AsRef<Path>>(proyect_path: T) -> Self {
        let path = proyect_path.as_ref().to_path_buf();
        let svn = SvnClient::new(&path);
        let status_lines = svn.svn_status();
        Self {
            running: true,
            proyect_path: path.clone(),
            svn: svn,
            status_lines,
            selected: 0,
            mode: AppMode::Normal,
            commit_message: "".to_string(),
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
        let info = ProjectInfo::new(self.proyect_path.to_string_lossy().to_string());
        let info_section = create_section_info(&info);
        let mut state = ListState::default().with_selected(Some(self.selected));
        let status_section =
            create_section_status(&self.status_lines, self.mode == AppMode::Normal);
        let selected_list = create_selected_items(&self.status_lines);
        let commit_section =
            create_section_commit(self.mode == AppMode::Commit, &self.commit_message);
        frame.render_widget(info_section, layout[0]);
        frame.render_stateful_widget(status_section, layout[1], &mut state);
        frame.render_widget(selected_list, layout[2]);
        frame.render_widget(commit_section, layout[3]);
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
        match self.mode {
            AppMode::Normal => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.selected = move_cursor_up(self.selected);
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.selected =
                        move_cursor_down(self.selected, self.status_lines.entries().len());
                }
                (_, KeyCode::Char('y')) => {
                    let _ = copy_file(self.selected, &self.status_lines.entries())
                        .expect("Error al copiar el archivo");
                }
                (_, KeyCode::Char(' ')) => {
                    self.status_lines.toggle_selection(self.selected);
                }
                (_, KeyCode::Char('c')) => self.mode = AppMode::Commit,
                _ => {}
            },
            AppMode::Commit => match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => self.mode = AppMode::Normal,
                (_, KeyCode::Enter) => {
                    push_basic_commit(&self.svn, &mut self.status_lines, &self.commit_message);
                    self.commit_message.clear();
                    self.mode = AppMode::Normal;
                }
                (_, KeyCode::Backspace) => {
                    self.commit_message.pop();
                }
                (_, KeyCode::Char(c)) => {
                    self.commit_message.push(c);
                }
                _ => {}
            },
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
