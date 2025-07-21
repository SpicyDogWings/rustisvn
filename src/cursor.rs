
pub fn move_cursor_down(selected: usize, lines: usize) -> usize {
    (selected + 1).min(lines.saturating_sub(1))
}

pub fn move_cursor_up(selected: usize) -> usize {
    selected.saturating_sub(1)
}
