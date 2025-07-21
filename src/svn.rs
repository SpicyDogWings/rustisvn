use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio}
};
use ratatui::style::{Color, Style};

#[derive(Debug)]
pub struct SvnClient {
    working_copy: PathBuf,
}

#[derive(Debug)]
pub struct StatusEntry {
    pub file: String,
    pub state: String,
}

impl SvnClient {
    pub fn new<T: AsRef<Path>>(working_copy: T) -> Self {
        SvnClient {
            working_copy: working_copy.as_ref().to_path_buf()
        }
    }

    pub fn raw_command(&self, args:&[&str]) -> String {
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

    pub fn svn_status(&self) -> Vec<StatusEntry> {
        let out = self.raw_command(&["status"]);
        out.lines()
            .filter_map(|line| {
                // svn status:  <estado><espacio><path>
                let mut parts = line.splitn(2, char::is_whitespace);
                let state = parts.next()?.to_string();
                let file  = parts.next()?.trim().to_string();
                Some(StatusEntry { state, file })
            }).collect()
    }
}

impl Default for SvnClient {
    fn default() -> Self {
        SvnClient::new(".")
    }
}

pub fn style_for_status(state: &str) -> Style {
    match state {
        "M" => Style::new().fg(Color::Blue),          // Modified
        "A" => Style::new().fg(Color::Green),         // Added
        "D" => Style::new().fg(Color::Red),           // Deleted
        "C" => Style::new().fg(Color::LightRed),      // Conflict
        "?" => Style::new().fg(Color::Yellow),        // Untracked
        "!" => Style::new().fg(Color::LightRed),      // Missing
        "I" => Style::new().fg(Color::DarkGray),      // Ignored
        "R" => Style::new().fg(Color::Cyan),          // Replaced
        "X" => Style::new().fg(Color::Magenta),       // External
        "~" => Style::new().fg(Color::LightMagenta),  // Obstructed
        _   => Style::new(),                          // Default
    }
}
