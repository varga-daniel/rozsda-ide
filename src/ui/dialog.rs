use gtk::*;
use std::path::PathBuf;

/// Egy becsomagolt GTKs FileChooserDialog, ami automatikusan a nyitásra állítja magát, és elpusztítja magát, amint droppolják.
pub struct OpenDialog(FileChooserDialog);

/// Egy becsomagolt GTKs FileChooserDialog, ami automatikusan a mentésre állítja magát, és elpusztítja magát, amint droppolják.
pub struct SaveDialog(FileChooserDialog);

impl OpenDialog {
    pub fn new(path: Option<PathBuf>) -> OpenDialog {
        let open_dialog = FileChooserDialog::new(
            Some("Megnyitás"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );

        open_dialog.add_button("Mégse", ResponseType::Cancel.into());
        open_dialog.add_button("Megnyit", ResponseType::Ok.into());

        let filter = FileFilter::new();
        filter.add_pattern("*.rs");
        filter.set_name("Rust forrásfájlok");

        open_dialog.add_filter(&filter);

        path.map(|p| open_dialog.set_current_folder(p));

        OpenDialog(open_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl SaveDialog {
    pub fn new(path: Option<PathBuf>) -> SaveDialog {
        let save_dialog = FileChooserDialog::new(
            Some("Mentés Másként"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        save_dialog.add_button("Mégse", ResponseType::Cancel.into());
        save_dialog.add_button("Mentés", ResponseType::Ok.into());

        let filter = FileFilter::new();
        filter.add_pattern("*.rs");
        filter.set_name("Rust forrásfájlok");

        save_dialog.add_filter(&filter);

        path.map(|p| save_dialog.set_current_folder(p));

        SaveDialog(save_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}
