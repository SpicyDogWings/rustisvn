use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Paragraph},
};

pub fn create_layout(frame: &Frame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(45),
            Constraint::Percentage(45),
        ])
        .split(frame.area())
        .to_vec()
}

pub struct ProjectInfo {
    path: String,
}

impl ProjectInfo {
    pub fn new(path: String) -> Self {
        ProjectInfo { path }
    }
}

pub fn create_section_info(info: &ProjectInfo) -> Paragraph {
    Paragraph::new(info.path.to_string()).block(Block::bordered().title("Project info"))
}
