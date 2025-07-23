use crate::svn::{SvnStatusList, style_for_status};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};

pub fn create_layout(frame: &Frame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
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
    Paragraph::new(Text::styled(
        info.path.to_string(),
        Style::default().fg(Color::Blue),
    ))
    .block(Block::bordered().title(" Project info "))
}

pub fn create_section_status(list: &SvnStatusList) -> List {
    let status_list: Vec<ListItem> = list
        .entries()
        .iter()
        .map(|entry| {
            let style = style_for_status(&entry.state());
            let line = Line::from(vec![
                Span::styled(String::from(&entry.state().to_string()), style),
                Span::raw(" "),
                Span::raw(entry.file().to_string_lossy()),
            ]);
            ListItem::new(line)
        })
        .collect();
    List::new(status_list)
        .block(
            Block::bordered()
                .title(" Status ")
                .border_style(Style::new().blue().bold())
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::new().bg(Color::DarkGray))
}

pub fn create_section_selected() -> Block {
    Block::bordered()
        .title(" Selected ".to_string())
        .border_type(BorderType::Rounded)
}
