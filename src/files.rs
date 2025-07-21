use crate::svn::StatusEntry;
use arboard::Clipboard;
use std::time::Duration;
use std::thread;

pub fn copy_file(selected: usize, list: &[StatusEntry]) -> Result<(), arboard::Error> {
    let file_selected = &list[selected].file;
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(file_selected)?;
    thread::sleep(Duration::from_millis(50));
    Ok(())
}
