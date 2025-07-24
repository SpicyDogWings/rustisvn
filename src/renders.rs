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
    .block(
        Block::bordered()
            .title(" Project info ")
            .border_style(Style::new())
            .border_type(BorderType::Rounded),
    )
}

pub fn create_section_status(list: &SvnStatusList) -> List {
    let status_list: Vec<ListItem> = list
        .entries()
        .iter()
        .enumerate()
        .map(|(i, _entry)| {
            let style_spans = create_status_line_spans(i, &list);
            let line = Line::from(style_spans);
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
        .highlight_style(Style::new().fg(Color::White).bg(Color::DarkGray))
}

pub fn create_selected_items(list: &SvnStatusList) -> List {
    let selected_items: Vec<ListItem> = list
        .selections()
        .iter()
        .filter_map(|&idx| list.entries().get(idx))
        .map(|entry| {
            let style = style_for_status(entry.state());
            let line = Line::from(vec![
                Span::styled(entry.state().to_string(), style),
                Span::raw(" "),
                Span::raw(entry.file().to_string_lossy()),
            ]);
            ListItem::new(line)
        })
        .collect();
    List::new(selected_items)
        .block(
            Block::bordered()
                .title(format!(" Selected "))
                .border_style(Style::new())
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::new().bg(Color::DarkGray))
}

pub fn create_status_line_spans(idx: usize, list: &SvnStatusList) -> Vec<Span> {
    if let Some(entry) = list.entries().get(idx) {
        let base_selected = Style::new().bg(Color::Blue).fg(Color::Black);
        let status_span = if list.selections().contains(&idx) {
            Span::styled(entry.state().to_string(), base_selected)
        } else {
            Span::styled(entry.state().to_string(), style_for_status(entry.state()))
        };
        let file_span = if list.selections().contains(&idx) {
            Span::styled(entry.file().to_string_lossy(), base_selected)
        } else {
            Span::raw(entry.file().to_string_lossy())
        };
        let divider_span = if list.selections().contains(&idx) {
            Span::styled(" ", base_selected)
        } else {
            Span::raw(" ")
        };
        vec![status_span, divider_span, file_span]
    } else {
        vec![
            Span::raw(format!("(Error: Índice {} inválido)", idx))
                .style(Style::new().fg(Color::Red)),
        ]
    }
}
