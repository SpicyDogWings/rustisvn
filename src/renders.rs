use crate::svn::{SvnStatusEntry, SvnStatusList, style_for_status};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Clear, List, ListItem, Paragraph, Wrap},
};

pub fn create_layout(frame: &Frame) -> Vec<Rect> {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Min(10),
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

#[derive(Debug, Clone)]
pub struct BlockRenderStatus {
    pub idx_selected: usize,
    pub error: bool,
}

impl BlockRenderStatus {
    pub fn new() -> Self {
        BlockRenderStatus {
            idx_selected: 0,
            error: false,
        }
    }
}

pub struct ProjectInfo {
    path: String,
}

impl ProjectInfo {
    pub fn new(path: String) -> Self {
        ProjectInfo { path }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct ModalInfo {
    pub title: String,
    pub message: String,
}

impl ModalInfo {
    pub fn new() -> Self {
        ModalInfo {
            title: "Void".to_string(),
            message: "".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum ModalType {
    #[default]
    Info,
    Warning,
    Error,
    None,
}

pub fn create_section_info(info: &ProjectInfo) -> Paragraph {
    Paragraph::new(Text::styled(
        info.path.to_string(),
        Style::default().fg(Color::Blue),
    ))
    .block(
        Block::bordered()
            .title(" Project info ")
            .border_style(Style::new().gray())
            .border_type(BorderType::Rounded),
    )
}

pub fn create_section_status(list: &SvnStatusList, is_error: bool, is_focused: bool) -> List {
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
    status_block = set_status_block(status_block, is_error, is_focused);
    List::new(status_list)
        .block(status_block)
        .highlight_style(Style::new().fg(Color::White).bg(Color::DarkGray))
}

pub fn create_selected_items(list: &SvnStatusList, is_error: bool, is_focused: bool) -> List {
    let mut selected_entries: Vec<&SvnStatusEntry> = list
        .selections
        .iter()
        .filter_map(|&idx| list.entries.get(idx))
        .collect();
    selected_entries.sort_by(|a, b| a.file.cmp(&b.file));
    let selected_items: Vec<ListItem> = selected_entries
        .into_iter()
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
    selected_block = set_status_block(selected_block, is_error, is_focused);
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

pub fn create_section_commit(commit_message: &str, is_error: bool, is_focused: bool) -> Paragraph {
    let mut commit_block = Block::bordered()
        .title(" Commit ")
        .border_type(BorderType::Rounded);
    commit_block = set_status_block(commit_block, is_error, is_focused);
    Paragraph::new(commit_message.to_string())
        .block(commit_block)
        .wrap(Wrap { trim: false })
}

pub fn set_status_block(block: Block, is_error: bool, is_focused: bool) -> Block {
    if is_error {
        block.border_style(Style::new().red().bold())
    } else if is_focused {
        block.border_style(Style::new().blue().bold())
    } else {
        block.border_style(Style::new().gray())
    }
}

pub fn centered_rect(r: Rect, text: &str, extra_height: u16) -> Rect {
    let total_width = r.width;
    let total_height = r.height;
    let target_width = (total_width as f32 * 0.75).round() as u16;
    let max_text_width = target_width.saturating_sub(4); // 4 for border and padding
    let max_text_width = max_text_width.max(1);
    let mut current_line_width: u16 = 0;
    let mut text_height: u16 = 1;
    for word in text.split(' ') {
        let word_width = word.len() as u16;
        if current_line_width.saturating_add(word_width) > max_text_width {
            text_height += 1;
            current_line_width = word_width.saturating_add(1); // +1 for the space
        } else {
            current_line_width = current_line_width
                .saturating_add(word_width)
                .saturating_add(1);
        }
    }
    let actual_width = target_width;
    let actual_height = text_height
        .saturating_add(extra_height)
        .saturating_add(2u16);
    let x = r.x + (total_width.saturating_sub(actual_width)) / 2;
    let y = r.y + (total_height.saturating_sub(actual_height)) / 2;
    Rect::new(x, y, actual_width, actual_height)
}

pub fn render_confirm_modal(frame: &mut Frame, title: &str, message: &str) {
    let area = centered_rect(frame.area(), message, 2);
    frame.render_widget(Clear, area);
    let outer_block = Block::bordered()
        .title(title)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue())
        .title_alignment(Alignment::Center);
    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);
    let modal_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Min(1)])
        .split(inner_area);
    let message_paraghap = Paragraph::new(message)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    frame.render_widget(message_paraghap, modal_layout[0]);
    let option_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Min(1)])
        .split(modal_layout[1]);
    let yes_text = Paragraph::new("Yes (y)")
        .style(Style::default().fg(Color::Blue))
        .alignment(Alignment::Center);
    frame.render_widget(yes_text, option_layout[0]);
    let no_text = Paragraph::new("No (n)")
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center);
    frame.render_widget(no_text, option_layout[1]);
}

pub fn render_modal(frame: &mut Frame, title: &str, message: &str, modal_type: ModalType) {
    let area = centered_rect(frame.area(), message, 0);
    frame.render_widget(Clear, area);
    let outer_block = Block::bordered()
        .title(title)
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center);
    let styled_block = set_modal_status(outer_block, modal_type);
    let modal_block = Paragraph::new(message)
        .block(styled_block)
        .wrap(Wrap { trim: false });
    frame.render_widget(modal_block, area);
}

pub fn set_modal_status(block: Block, modal_type: ModalType) -> Block {
    let mut style = Style::default();
    style = match modal_type {
        ModalType::Info => style.blue(),
        ModalType::Warning => style.yellow(),
        ModalType::Error => style.red(),
        _ => style,
    };
    block.border_style(style)
}
