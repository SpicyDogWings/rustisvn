mod cursor;
mod files;
mod renders;
mod svn;
use crate::{
    cursor::{move_cursor_down, move_cursor_up},
    files::copy_file,
    renders::{
        BlockRenderStatus, ModalInfo, ModalType, ProjectInfo, create_layout, create_section_commit,
        create_section_info, create_section_status, create_selected_items, render_confirm_modal,
        render_modal,
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
pub enum ConfirmMode {
    #[default]
    Revert,
}

#[derive(Debug, Default, PartialEq)]
pub enum AppMode {
    #[default]
    Normal,
    Commit,
    Selections,
    Confirm(ConfirmMode),
    Modal(ModalType),
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    directory: PathBuf,
    svn: SvnClient,
    block_status: Vec<BlockRenderStatus>,
    mode: AppMode,
    modal: ModalInfo,
}

impl App {
    pub fn new<T: AsRef<Path>>(directory: T) -> Self {
        let path = directory.as_ref().to_path_buf();
        let mut svn = SvnClient::new(&path);
        svn.init_svn_status();
        let block_status = vec![BlockRenderStatus::new(); 3];
        let modal = ModalInfo::new();
        Self {
            running: true,
            directory: path.clone(),
            svn,
            block_status,
            mode: AppMode::Normal,
            modal,
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
        let mut state = ListState::default().with_selected(Some(self.block_status[0].idx_selected));
        let mut state_selected_list =
            ListState::default().with_selected(Some(self.block_status[1].idx_selected));
        let status_section =
            create_section_status(&self.svn.status, false, self.mode == AppMode::Normal);
        let selected_list =
            create_selected_items(&self.svn.status, false, self.mode == AppMode::Selections);
        let commit_section = create_section_commit(
            &self.svn.status.commit_message(),
            self.block_status[2].error,
            self.mode == AppMode::Commit,
        );
        frame.render_widget(info_section, layout[0]);
        frame.render_stateful_widget(status_section, layout[1], &mut state);
        frame.render_stateful_widget(selected_list, layout[2], &mut state_selected_list);
        frame.render_widget(commit_section, layout[3]);

        if let AppMode::Confirm(confirm_type) = &self.mode {
            let (title, message) = match confirm_type {
                ConfirmMode::Revert => (
                    " Confirmar Revertir ",
                    "¿Estás seguro de que quieres revertir los cambios?",
                ),
            };
            render_confirm_modal(frame, title, message);
        }
        if let AppMode::Modal(modal_type) = &self.mode {
            render_modal(
                frame,
                &self.modal.title,
                &self.modal.message,
                modal_type.clone(),
            );
        }
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
        match &self.mode {
            AppMode::Normal => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.block_status[0].idx_selected =
                        move_cursor_up(self.block_status[0].idx_selected);
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.block_status[0].idx_selected = move_cursor_down(
                        self.block_status[0].idx_selected,
                        self.svn.status.entries.len(),
                    );
                }
                (_, KeyCode::Char('y')) => {
                    let _ = copy_file(self.block_status[0].idx_selected, &self.svn.status.entries)
                        .expect("Error al copiar el archivo");
                }
                (_, KeyCode::Char('u')) => {
                    self.svn.refresh_svn_status();
                }
                (_, KeyCode::Char('a')) => {
                    self.svn.add_to_svn(self.block_status[0].idx_selected);
                }
                (_, KeyCode::Char('r')) => {
                    self.mode = AppMode::Confirm(ConfirmMode::Revert);
                }
                (_, KeyCode::Char(' ')) => {
                    self.svn
                        .status
                        .toggle_selection(self.block_status[0].idx_selected);
                }
                (_, KeyCode::Char('c')) => {
                    self.mode = AppMode::Commit;
                    self.block_status[2].error = false;
                }
                (_, KeyCode::Char('s')) => self.mode = AppMode::Selections,
                _ => {}
            },
            AppMode::Commit => match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => {
                    self.mode = AppMode::Normal;
                }
                (_, KeyCode::Enter) => {
                    match self.svn.push_basic_commit() {
                        Ok(_) => {
                            self.block_status[2].error = false;
                            self.svn.status.clear_commit_message();
                            self.mode = AppMode::Normal;
                        }
                        Err(error_message) => {
                            self.block_status[2].error = true;
                            self.modal.title = " Error de Commit ".to_string();
                            self.modal.message = error_message;
                            self.mode = AppMode::Modal(ModalType::Error);
                        }
                    }
                    self.svn.status.clear_commit_message();
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
                    self.block_status[2].error = false;
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.block_status[1].idx_selected =
                        move_cursor_up(self.block_status[1].idx_selected);
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.block_status[1].idx_selected = move_cursor_down(
                        self.block_status[1].idx_selected,
                        self.svn.status.selections.len(),
                    );
                }
                (_, KeyCode::Char(' ')) => {
                    self.svn
                        .status
                        .toggle_selection_by_file(self.block_status[1].idx_selected);
                }
                _ => {}
            },
            AppMode::Confirm(action_type) => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('n')) => {
                    self.mode = AppMode::Normal;
                }
                (_, KeyCode::Char('y')) => {
                    match action_type {
                        ConfirmMode::Revert => {
                            self.svn.revert_to_svn(self.block_status[0].idx_selected);
                        }
                    }
                    self.mode = AppMode::Normal;
                }
                _ => {}
            },
            AppMode::Modal(_) => match (key.modifiers, key.code) {
                (_, KeyCode::Enter | KeyCode::Esc) => {
                    self.mode = AppMode::Normal;
                }
                _ => {}
            },
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
