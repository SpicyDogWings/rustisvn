use ratatui::style::{Color, Style};
use std::collections::HashSet;
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub struct SvnClient {
    working_copy: PathBuf,
}

#[derive(Debug)]
pub struct StatusEntry {
    file: PathBuf,
    state: String,
}

impl StatusEntry {
    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn state(&self) -> &String {
        &self.state
    }
}

#[derive(Debug, Default)]
pub struct SvnStatusList {
    entries: Vec<StatusEntry>,
    selections: HashSet<usize>,
}

impl SvnStatusList {
    pub fn new(entries: Vec<StatusEntry>, selections: HashSet<usize>) -> Self {
        SvnStatusList {
            entries,
            selections,
        }
    }

    pub fn entries(&self) -> &Vec<StatusEntry> {
        &self.entries
    }

    pub fn selections(&self) -> &HashSet<usize> {
        &self.selections
    }

    pub fn toggle_selection(&mut self, idx: usize) {
        if self.selections.contains(&idx) {
            self.selections.remove(&idx);
        } else {
            self.selections.insert(idx);
        }
    }
}

impl SvnClient {
    pub fn new<T: AsRef<Path>>(working_copy: T) -> Self {
        SvnClient {
            working_copy: working_copy.as_ref().to_path_buf(),
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
                Some(StatusEntry { state, file })
            })
            .collect();
        SvnStatusList::new(entries, HashSet::new())
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
