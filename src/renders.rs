use crate::svn::{SvnStatusList, style_for_status};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, List, ListItem, Paragraph, Wrap},
};

pub fn create_layout(frame: &Frame) -> Vec<Rect> {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Fill(5),
            Constraint::Min(7),
        ])
        .split(frame.area());
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[2]);
    vec![
        main_chunks[0],
        main_chunks[1],
        horizontal_chunks[0],
        horizontal_chunks[1],
    ]
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

pub fn create_section_status(list: &SvnStatusList, is_focused: bool) -> List {
    let status_list: Vec<ListItem> = list
        .entries
        .iter()
        .enumerate()
        .map(|(i, _entry)| {
            let style_spans = create_status_line_spans(i, &list);
            let line = Line::from(style_spans);
            ListItem::new(line)
        })
        .collect();
    let mut status_block = Block::bordered()
        .title(" Status ")
        .border_type(BorderType::Rounded);
    status_block = is_focused_styles(is_focused, status_block);
    List::new(status_list)
        .block(status_block)
        .highlight_style(Style::new().fg(Color::White).bg(Color::DarkGray))
}

pub fn create_selected_items(list: &SvnStatusList, is_focused: bool) -> List {
    let selected_items: Vec<ListItem> = list
        .selections
        .iter()
        .filter_map(|&idx| list.entries.get(idx))
        .map(|entry| {
            let style = style_for_status(&entry.state);
            let line = Line::from(vec![
                Span::styled(entry.state.to_string(), style),
                Span::raw(" "),
                Span::raw(entry.file.to_string_lossy()),
            ]);
            ListItem::new(line)
        })
        .collect();
    let mut selected_block = Block::bordered()
        .title(" Selected ")
        .border_style(Style::new())
        .border_type(BorderType::Rounded);
    selected_block = is_focused_styles(is_focused, selected_block);
    List::new(selected_items)
        .block(selected_block)
        .highlight_style(Style::new().bg(Color::DarkGray))
}

pub fn create_status_line_spans(idx: usize, list: &SvnStatusList) -> Vec<Span> {
    if let Some(entry) = list.entries.get(idx) {
        let base_selected = Style::new().bg(Color::Blue).fg(Color::Black);
        let status_span = if list.selections.contains(&idx) {
            Span::styled(entry.state.to_string(), base_selected)
        } else {
            Span::styled(entry.state.to_string(), style_for_status(&entry.state))
        };
        let file_span = if list.selections.contains(&idx) {
            Span::styled(entry.file.to_string_lossy(), base_selected)
        } else {
            Span::raw(entry.file.to_string_lossy())
        };
        let divider_span = if list.selections.contains(&idx) {
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

pub fn create_section_commit(is_focused: bool, commit_message: &str) -> Paragraph {
    let mut commit_block = Block::bordered()
        .title(" Commit ")
        .border_type(BorderType::Rounded);
    commit_block = is_focused_styles(is_focused, commit_block);
    Paragraph::new(commit_message.to_string())
        .block(commit_block)
        .wrap(Wrap { trim: false })
}

pub fn is_focused_styles(on_focus: bool, block: Block) -> Block {
    if on_focus {
        block.border_style(Style::new().blue().bold())
    } else {
        block.border_style(Style::new().gray())
    }
}
