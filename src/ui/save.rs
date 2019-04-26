use super::misc::*;
use super::SaveDialog;
use crate::state::ActiveMetadata;
use gtk::*;
use sourceview::*;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::RwLock;

pub enum SaveAction {
    New(ActiveMetadata),
    Saved,
    Canceled,
}

pub fn save(
    editor: &Buffer,
    save: &MenuItem,
    current_file: &RwLock<Option<ActiveMetadata>>,
    save_as: bool,
) {
    if let Some(text) = get_buffer(editor) {
        // Ha másként mentünk, akkor nyilván nem hívjuk be a jelenlegi fájl helyét a mentés helyeként, így kikényszerítve a write_data funkciót, hogy bekérjen egy helyet.
        // Ellenkező esetben írjunk a meglévő fájlba, ha lehetséges.
        let result = if save_as {
            write_data(None, text.as_bytes())
        } else {
            write_data(current_file.read().unwrap().as_ref(), text.as_bytes())
        };

        // Majd leellenőrizzük, hogy mi volt a mentés eredménye.
        // Nagyon cselekedni csak akkor kell, ha Ok(Some(ActiveMetadata))-t kaptunk vissza.
        match result {
            Ok(SaveAction::New(file)) => {
                let mut current_file = current_file.write().unwrap();
                *current_file = Some(file);
                save.set_sensitive(false);
            }
            Ok(SaveAction::Saved) => {
                // De azért egy egyszerű mentésnél is jelezzük a felhasználónak, hogy nincs különbség a mentett fájl és az éppen látható között.
                if let Some(ref mut current_file) = *current_file.write().unwrap() {
                    current_file.set_sum(&text.as_bytes());
                    save.set_sensitive(false);
                }
            }
            // Más esetben semmi.
            _ => (),
        }
    }
}

/// Elmenti a megadott adatot a megadott fájlba. Ha az ösvény **None**, akkor meghívja a mentési ablakot,
/// hogy bekérjen a felhasználótól egy fájlhelyet. Ebben az esetben a függvény
/// **Ok(Some(path))**-fal tér vissza, ellenkező esetben **Ok(None)**-nal.
/// Az **Err** visszatérési érték valamilyen IO hibára utal a fájlmentés közben.
fn write_data(path: Option<&ActiveMetadata>, data: &[u8]) -> io::Result<SaveAction> {
    if let Some(path) = path {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.get_path())?;
        file.write_all(&data)?;
        return Ok(SaveAction::Saved);
    }

    let save_dialog = SaveDialog::new(None);
    if let Some(new_path) = save_dialog.run() {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&new_path)?;
        file.write_all(data)?;
        Ok(SaveAction::New(ActiveMetadata::new(new_path, data)))
    } else {
        Ok(SaveAction::Canceled)
    }
}
