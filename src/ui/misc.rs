use glib::*;
use gtk::*;
use sourceview::*;
use std::path::Path;

/// Beállítja a header címét a fájlnévre.
pub fn set_title(headerbar: &HeaderBar, path: &Path) {
    if let Some(filename) = path.file_name() {
        let filename: &str = &filename.to_string_lossy();
        headerbar.set_title(filename);
    }
}

/// Visszaadja egy szövegbuffer teljes belsejét.
pub fn get_buffer(buffer: &Buffer) -> Option<GString> {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    buffer.get_text(&start, &end, true)
}
