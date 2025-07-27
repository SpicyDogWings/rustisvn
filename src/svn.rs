use ratatui::style::{Color, Style};
use std::collections::HashSet;
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct SvnStatusEntry {
    pub file: PathBuf,
    pub state: String,
}

impl SvnStatusEntry {
    pub fn new(file: PathBuf, state: String) -> Self {
        SvnStatusEntry { file, state }
    }
}

#[derive(Debug, Default)]
pub struct SvnStatusList {
    pub entries: Vec<SvnStatusEntry>,
    pub selections: HashSet<usize>,
    commit_message: String,
}

impl SvnStatusList {
    pub fn new(entries: Vec<SvnStatusEntry>, selections: HashSet<usize>) -> Self {
        SvnStatusList {
            entries,
            selections,
            commit_message: String::new(),
        }
    }

    pub fn commit_message(&self) -> &str {
        &self.commit_message
    }

    pub fn toggle_selection(&mut self, idx: usize) {
        if self.selections.contains(&idx) {
            self.selections.remove(&idx);
        } else {
            self.selections.insert(idx);
        }
    }

    pub fn toggle_selection_by_file(&mut self, idx_selected: usize) {
        let file_to_remove = self
            .selections
            .iter()
            .filter_map(|&idx| self.entries.get(idx))
            .nth(idx_selected)
            .map(|entry| entry.file.to_path_buf());
        if let Some(file) = file_to_remove {
            if let Some(idx) = self.entries.iter().position(|entry| entry.file == file) {
                self.selections.remove(&idx);
            }
        }
    }

    pub fn clear_commit_message(&mut self) {
        self.commit_message.clear();
    }

    pub fn push_char_to_commit_message(&mut self, c: char) {
        self.commit_message.push(c);
    }

    pub fn pop_char_from_commit_message(&mut self) {
        self.commit_message.pop();
    }
}

#[derive(Debug)]
pub struct SvnClient {
    working_copy: PathBuf,
    pub status: SvnStatusList,
}

impl SvnClient {
    pub fn new<T: AsRef<Path>>(working_copy: T) -> Self {
        SvnClient {
            working_copy: working_copy.as_ref().to_path_buf(),
            status: SvnStatusList::new(Vec::new(), HashSet::new()),
        }
    }

    pub fn raw_command(&self, args: &[&str]) -> String {
        let out = Command::new("svn")
            .args(args)
            .current_dir(&self.working_copy)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match out {
            Ok(o) => String::from_utf8_lossy(&o.stdout).into_owned(), // acá se ve el mensaje de notting to update i think
            Err(_) => String::new(),
        }
    }

    pub fn svn_status(&mut self) {
        let out = self.raw_command(&["status"]);
        let entries = out
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, char::is_whitespace);
                let state = parts.next()?.to_string();
                let file = PathBuf::from(parts.next()?.to_string().trim());
                Some(SvnStatusEntry::new(file, state))
            })
            .collect();
        self.status = SvnStatusList::new(entries, HashSet::new());
    }

    pub fn push_basic_commit(&mut self) {
        let mut args = vec!["commit", "-m", self.status.commit_message()];
        let file_args: Vec<&str> = self
            .status
            .selections
            .iter()
            .filter_map(|&idx| self.status.entries.get(idx))
            .filter_map(|entry| entry.file.to_str())
            .collect();
        if file_args.is_empty() {
            return;
        };
        args.extend(file_args);
        self.raw_command(&args);
        self.svn_status();
    }
}

impl Default for SvnClient {
    fn default() -> Self {
        SvnClient::new(".")
    }
}

pub fn style_for_status(state: &str) -> Style {
    match state {
        "M" => Style::new().fg(Color::Blue),         // Modified
        "A" => Style::new().fg(Color::Green),        // Added
        "D" => Style::new().fg(Color::Red),          // Deleted
        "C" => Style::new().fg(Color::LightRed),     // Conflict
        "?" => Style::new().fg(Color::Yellow),       // Untracked
        "!" => Style::new().fg(Color::LightRed),     // Missing
        "I" => Style::new().fg(Color::DarkGray),     // Ignored
        "R" => Style::new().fg(Color::Cyan),         // Replaced
        "X" => Style::new().fg(Color::Magenta),      // External
        "~" => Style::new().fg(Color::LightMagenta), // Obstructed
        _ => Style::new(),                           // Default
    }
}
