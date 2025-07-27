use ratatui::style::{Color, Style};
use std::collections::HashSet;
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct SvnStatusEntry {
    file: PathBuf,
    state: String,
}

impl SvnStatusEntry {
    pub fn new(file: PathBuf, state: String) -> Self {
        SvnStatusEntry { file, state }
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn state(&self) -> &String {
        &self.state
    }
}

#[derive(Debug, Default)]
pub struct SvnStatusList {
    entries: Vec<SvnStatusEntry>,
    selections: HashSet<usize>,
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

    pub fn entries(&self) -> &Vec<SvnStatusEntry> {
        &self.entries
    }

    pub fn selections(&self) -> &HashSet<usize> {
        &self.selections
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
            .selections()
            .iter()
            .filter_map(|&idx| self.entries().get(idx))
            .nth(idx_selected)
            .map(|entry| entry.file().to_path_buf());
        if let Some(file) = file_to_remove {
            if let Some(idx) = self
                .entries()
                .iter()
                .position(|entry| entry.file() == &file)
            {
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
}

impl SvnClient {
    pub fn new<T: AsRef<Path>>(working_copy: T) -> Self {
        SvnClient {
            working_copy: working_copy.as_ref().to_path_buf(),
            // SvnStatusList
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

    pub fn svn_status(&self) -> SvnStatusList {
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
        SvnStatusList::new(entries, HashSet::new())
    }

    pub fn push_basic_commit(&self, status_list: &mut SvnStatusList) {
        let mut args = vec!["commit", "-m", status_list.commit_message()];
        let file_args: Vec<&str> = status_list
            .selections()
            .iter()
            .filter_map(|&idx| status_list.entries().get(idx))
            .filter_map(|entry| entry.file().to_str())
            .collect();
        if file_args.is_empty() {
            return;
        };
        args.extend(file_args);
        self.raw_command(&args);
        *status_list = self.svn_status();
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
