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
#[command(
    about = "Una TUI para proyectos con Svn",
    long_about = "
Rustisvn

Una app minimalista y simple para usar en projectos svn.

Funcionalidades disponibles como:
- Seleccionar archivo
- Hacer commits
- Copiar el path del archivo

Lo necesario para un flujo simple y sencillo de trabajo en la rama trunck"
)]
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
    SelectedList,
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    proyect_path: PathBuf,
    svn: SvnClient,
    status_lines: SvnStatusList,
    selected: usize,
    list_selected: usize,
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
            svn,
            status_lines,
            selected: 0,
            list_selected: 0,
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
        let mut state_selected_list = ListState::default().with_selected(Some(self.list_selected));
        let status_section =
            create_section_status(&self.status_lines, self.mode == AppMode::Normal);
        let selected_list =
            create_selected_items(&self.status_lines, self.mode == AppMode::SelectedList);
        let commit_section =
            create_section_commit(self.mode == AppMode::Commit, &self.commit_message);
        frame.render_widget(info_section, layout[0]);
        frame.render_stateful_widget(status_section, layout[1], &mut state);
        frame.render_stateful_widget(selected_list, layout[2], &mut state_selected_list);
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
                (_, KeyCode::Char('s')) => self.mode = AppMode::SelectedList,
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
            AppMode::SelectedList => match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => self.mode = AppMode::Normal,
                (_, KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                (_, KeyCode::Char('c')) => self.mode = AppMode::Commit,
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.list_selected = move_cursor_up(self.list_selected);
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.list_selected =
                        move_cursor_down(self.list_selected, self.status_lines.selections().len());
                }
                (_, KeyCode::Char(' ')) => {
                    let file_to_remove: Option<PathBuf> = self
                        .status_lines
                        .selections()
                        .iter()
                        .filter_map(|&idx| self.status_lines.entries().get(idx))
                        .nth(self.list_selected)
                        .map(|entry| entry.file().to_path_buf());
                    if let Some(file) = file_to_remove {
                        self.status_lines.toggle_selection_by_file(&file);
                    }
                }
                _ => {}
            },
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
