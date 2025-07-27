use crate::svn::SvnStatusEntry;
use arboard::Clipboard;
use std::thread;
use std::time::Duration;

pub fn copy_file(selected: usize, list: &[SvnStatusEntry]) -> Result<(), arboard::Error> {
    let file_selected = &list[selected].file();
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(file_selected.to_string_lossy())?;
    thread::sleep(Duration::from_millis(50));
    Ok(())
}
