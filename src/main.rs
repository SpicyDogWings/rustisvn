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
    svn::SvnClient,
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
    let directory = canonicalize(&args.directory).unwrap();
    let result = App::new(directory).run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default, PartialEq)]
pub enum AppMode {
    #[default]
    Normal,
    Commit,
    Selections,
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    directory: PathBuf,
    svn: SvnClient,
    idx_selected_sl: usize,
    idx_selected_slist: usize,
    status_error: bool,
    mode: AppMode,
}

impl App {
    pub fn new<T: AsRef<Path>>(directory: T) -> Self {
        let path = directory.as_ref().to_path_buf();
        let mut svn = SvnClient::new(&path);
        svn.svn_status();
        Self {
            running: true,
            directory: path.clone(),
            svn,
            idx_selected_sl: 0,
            idx_selected_slist: 0,
            status_error: false,
            mode: AppMode::Normal,
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
        let info = ProjectInfo::new(self.directory.to_string_lossy().to_string());
        let info_section = create_section_info(&info);
        let mut state = ListState::default().with_selected(Some(self.idx_selected_sl));
        let mut state_selected_list =
            ListState::default().with_selected(Some(self.idx_selected_slist));
        let status_section =
            create_section_status(&self.svn.status, false, self.mode == AppMode::Normal);
        let selected_list =
            create_selected_items(&self.svn.status, false, self.mode == AppMode::Selections);
        let commit_section = create_section_commit(
            &self.svn.status.commit_message(),
            self.status_error,
            self.mode == AppMode::Commit,
        );
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
                    self.idx_selected_sl = move_cursor_up(self.idx_selected_sl);
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.idx_selected_sl =
                        move_cursor_down(self.idx_selected_sl, self.svn.status.entries.len());
                }
                (_, KeyCode::Char('y')) => {
                    let _ = copy_file(self.idx_selected_sl, &self.svn.status.entries)
                        .expect("Error al copiar el archivo");
                }
                (_, KeyCode::Char(' ')) => {
                    self.svn.status.toggle_selection(self.idx_selected_sl);
                }
                (_, KeyCode::Char('c')) => {
                    self.mode = AppMode::Commit;
                    self.status_error = false;
                }
                (_, KeyCode::Char('s')) => self.mode = AppMode::Selections,
                _ => {}
            },
            AppMode::Commit => match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => {
                    self.mode = AppMode::Normal;
                }
                (_, KeyCode::Enter) => {
                    self.status_error = !self.svn.push_basic_commit();
                    self.svn.status.clear_commit_message();
                    self.mode = AppMode::Normal;
                }
                (_, KeyCode::Backspace) => {
                    self.svn.status.pop_char_from_commit_message();
                }
                (_, KeyCode::Char(c)) => {
                    self.svn.status.push_char_to_commit_message(c);
                }
                _ => {}
            },
            AppMode::Selections => match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => self.mode = AppMode::Normal,
                (_, KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                (_, KeyCode::Char('c')) => {
                    self.mode = AppMode::Commit;
                    self.status_error = false;
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.idx_selected_slist = move_cursor_up(self.idx_selected_slist);
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.idx_selected_slist =
                        move_cursor_down(self.idx_selected_slist, self.svn.status.selections.len());
                }
                (_, KeyCode::Char(' ')) => {
                    self.svn
                        .status
                        .toggle_selection_by_file(self.idx_selected_slist);
                }
                _ => {}
            },
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
