use super::misc::*;
use super::save::save;
use super::{Content, Header, OpenDialog};
use crate::state::ActiveMetadata;
use gdk::enums::key;
use gdk::ModifierType;
use gtk;
use gtk::*;
use std::fs::File;
use std::io::Read;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

/// Egy becsomagolt `App`.
pub struct ConnectedApp(App);

impl ConnectedApp {
    /// Kirajzolja az ablakot, majd elindítja a GTK loop-ját.
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}

impl App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            eprintln!("A GTK inicializációja megbukott!");
            process::exit(1);
        }

        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(&header.container);
        window.set_title("Rozsda IDE");
        window.set_default_size(800, 600);

        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
        }
    }

    /// Ez hozza létre a `ConnectedApp` struktúrát, amivel aztán tényleg dolgozunk.
    pub fn connect_events(self) -> ConnectedApp {
        // Ezt a kettőt Arc-ba csomagoljuk, hogy a szálakon keresztül is biztonságosan elérhetőek legyenek.

        // Továbbá egy írás-olvasás zárba csomagoljuk a jelenlegi fájlt.
        let current_file = Arc::new(RwLock::new(None));

        // A teljes képernyősséget viszont egyszerűen csak egy bool-ként tároljuk.
        let fullscreen = Arc::new(AtomicBool::new(false));

        {
            let save = &self.header.save;
            let save_as = &self.header.save_as;

            self.open_file(current_file.clone());
            self.save_event(&save, &save, current_file.clone(), false);
            self.save_event(&save, &save_as, current_file.clone(), true);
            self.key_events(current_file, fullscreen);
        }

        ConnectedApp(self)
    }

    /// Hozzákapcsol megadott billentyűkombinációkhoz / billentyűkhöz parancsokat a GDK
    /// által adott lehetőségekkel.
    fn key_events(
        &self,
        current_file: Arc<RwLock<Option<ActiveMetadata>>>,
        fullscreen: Arc<AtomicBool>,
    ) {
        // Bár kellenek nekünk a referenciák, nem akarunk velük mit csinálni, ezért klónozunk.
        let editor = self.content.source.buff.clone();
        let headerbar = self.header.container.clone();
        let save_button = self.header.save.clone();

        // Minden gomblenyomás meghívja ezt a részt.
        self.window.connect_key_press_event(move |window, gdk| {
            match gdk.get_keyval() {
                // Teljes képernyő F11 esetén.
                key::F11 => {
                    if fullscreen.fetch_xor(true, Ordering::SeqCst) {
                        window.unfullscreen();
                    } else {
                        window.fullscreen();
                    }
                }
                // Mentés akkor ha Ctrl+S-et kapunk.
                key if key == 's' as u32
                    && gdk.get_state().contains(ModifierType::CONTROL_MASK) =>
                {
                    save(&editor, &headerbar, &save_button, &current_file, false);
                }
                // Semmi egyébként.
                _ => (),
            }
            Inhibit(false)
        });
    }

    /// A Megnyitás gomb beállítása fájlok megnyitására.
    fn open_file(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
        let editor = self.content.source.buff.clone();
        let headerbar = self.header.container.clone();
        self.header.open.connect_clicked(move |_| {
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
                    let mut contents = String::new();
                    let _ = file.read_to_string(&mut contents);

                    // Frissítsük a címet, mivel új fájlt töltöttünk be.
                    set_title(&headerbar, &new_file);
                    if let Some(parent) = new_file.parent() {
                        let subtitle: &str = &parent.to_string_lossy();
                        headerbar.set_subtitle(subtitle);
                    }

                    // Frissítsük a jelenlegi fájl változóját.
                    *current_file.write().unwrap() =
                        Some(ActiveMetadata::new(new_file, &contents.as_bytes()));

                    // Végül ne felejtsük el betenni a tartalmat az editorba!
                    editor.set_text(&contents);
                }
            }
        });
    }

    /// A save funkció összekötése a megadott gombbal.
    fn save_event(
        &self,
        save_button: &Button,
        actual_button: &Button,
        current_file: Arc<RwLock<Option<ActiveMetadata>>>,
        save_as: bool,
    ) {
        let editor = self.content.source.buff.clone();
        let headerbar = self.header.container.clone();
        let save_button = save_button.clone();
        actual_button.connect_clicked(move |_| {
            save(&editor, &headerbar, &save_button, &current_file, save_as)
        });
    }
}
