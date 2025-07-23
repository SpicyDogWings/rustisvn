use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
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
