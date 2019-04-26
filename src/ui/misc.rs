use super::dialog::*;
use super::header::*;
use super::save::*;
use crate::state::*;
use glib::GString;
use gtk::*;
use sourceview::*;

use std::fs::File;
use std::io::Read;
use std::sync::RwLock;

/// Visszaadja egy szövegbuffer teljes belsejét.
pub fn get_buffer(buffer: &Buffer) -> Option<GString> {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    buffer.get_text(&start, &end, true)
}

pub fn open_file(editor: &Buffer, current_file: &RwLock<Option<ActiveMetadata>>) {
    // Ha létezik, használjuk fel a jelenlegi fájl szülő könyvtárát az OpenDialog
    // kiindulópontjaként.
    let open_dialog = OpenDialog::new({
        let lock = current_file.read().unwrap();
        if let Some(ref path) = *lock {
            path.get_dir()
        } else {
            None
        }
    });

    // Majd futtassuk az OpenDialog-ot, és ha kiválasztott a
    // felhasználó egy fájlt, nyissuk meg.
    if let Some(new_file) = open_dialog.run() {
        if let Ok(mut file) = File::open(&new_file) {
            // Ha sikeres volt a fájl megnyitása, olvassuk be a tartalmát a bufferbe.
            let mut contents = std::string::String::new();
            let _ = file.read_to_string(&mut contents);

            // Frissítsük a címet, mivel új fájlt töltöttünk be.
            let mut subtitle = String::new();
            if let Some(parent) = new_file.parent() {
                subtitle = String::from(parent.to_str().unwrap());
            }

            // Frissítsük a jelenlegi fájl változóját.
            *current_file.write().unwrap() =
                Some(ActiveMetadata::new(new_file, &contents.as_bytes()));

            // Végül ne felejtsük el betenni a tartalmat az editorba!
            editor.set_text(&contents);
        }
    }
}

pub fn close_file(
    window: &Window,
    editor: &Buffer,
    save_item: &MenuItem,
    current_file: &RwLock<Option<ActiveMetadata>>,
) {
    let mut unsaved_changes = false;

    if let Some(source_code) = get_buffer(&editor) {
        if source_code.len() > 0 {
            unsaved_changes = true;
        }
        if let Some(ref current_file) = *current_file.read().unwrap() {
            unsaved_changes = !current_file.is_same_as(&source_code.as_bytes());
        }
    }

    if unsaved_changes {
        let response = ask_about_unsaved_changes(window);
        if response == ResponseType::Yes.into() {
            save(&editor, &save_item, &current_file, false);
        }
    }

    *current_file.write().unwrap() = None;

    editor.delete(&mut editor.get_start_iter(), &mut editor.get_end_iter());
}

pub fn ask_about_unsaved_changes(parent: &Window) -> i32 {
    let dialog = MessageDialog::new(
        Some(parent),
        DialogFlags::DESTROY_WITH_PARENT,
        MessageType::Question,
        ButtonsType::None,
        "Szeretné menteni a módosításait?",
    );

    dialog.set_title("Mentetlen módosítások");
    dialog.add_button("Mentés", ResponseType::Yes);
    dialog.add_button("Elvetés", ResponseType::No);

    let result = dialog.run();
    dialog.close();

    result
}

pub fn open_project(current_project: &RwLock<Option<ProjectMetadata>>) {
    let open_dialog = OpenFolderDialog::new(None);

    if let Some(new_project) = open_dialog.run() {
        if new_project.is_dir() {
            *current_project.write().unwrap() = Some(ProjectMetadata::new(new_project));
        }
    }
}
