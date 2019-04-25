use super::misc::*;
use super::save::save;
use super::{Content, Header};
use crate::state::ActiveMetadata;
use gdk::enums::key;
use gdk::ModifierType;
use gtk;
use gtk::*;
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

        let save = &self.header.file_menu.save_item;
        let save_as = &self.header.file_menu.save_as_item;
        let close = &self.header.file_menu.close_file_item;
        let quit = &self.header.file_menu.quit_item;

        self.editor_changed(current_file.clone(), &save);
        self.open_event(current_file.clone());
        self.save_event(&save, &save, current_file.clone(), false);
        self.save_event(&save, &save_as, current_file.clone(), true);
        self.close_file_event(&save, &close, current_file.clone());
        self.close_file_event(&save, &quit, current_file.clone());
        self.quit_event(&quit);
        self.key_events(current_file, fullscreen);

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
        let save_item = self.header.file_menu.save_item.clone();

        // Minden gomblenyomás meghívja ezt a részt.
        self.window
            .connect_key_press_event(move |window, eventkey| {
                match eventkey.get_keyval() {
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
                        && eventkey.get_state().contains(ModifierType::CONTROL_MASK) =>
                    {
                        save(&editor, &headerbar, &save_item, &current_file, false);
                    }
                    // Megnyitás a Ctrl+O-val.
                    key if key == 'o' as u32
                        && eventkey.get_state().contains(ModifierType::CONTROL_MASK) =>
                    {
                        open_file(&editor, &headerbar, &current_file);
                    }
                    // Fájl bezárása Ctrl+W-val.
                    key if key == 'w' as u32
                        && eventkey.get_state().contains(ModifierType::CONTROL_MASK) =>
                    {
                        close_file(&window, &editor, &headerbar, &save_item, &current_file);
                    }
                    // Kilépés Ctrl+Q-val.
                    key if key == 'q' as u32
                        && eventkey.get_state().contains(ModifierType::CONTROL_MASK) =>
                    {
                        close_file(&window, &editor, &headerbar, &save_item, &current_file);
                        main_quit();
                    }
                    // Semmi egyébként.
                    _ => (),
                }
                Inhibit(false)
            });
    }

    /// A Megnyitás gomb beállítása fájlok megnyitására.
    fn open_event(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
        let editor = self.content.source.buff.clone();
        let headerbar = self.header.container.clone();
        self.header.file_menu.open_item.connect_activate(move |_| {
            open_file(&editor, &headerbar, &current_file);
        });
    }

    /// A save funkció összekötése a megadott gombbal.
    fn save_event(
        &self,
        save_item: &MenuItem,
        actual_item: &MenuItem,
        current_file: Arc<RwLock<Option<ActiveMetadata>>>,
        save_as: bool,
    ) {
        let editor = self.content.source.buff.clone();
        let headerbar = self.header.container.clone();
        let save_item = save_item.clone();
        actual_item.connect_activate(move |_| {
            save(&editor, &headerbar, &save_item, &current_file, save_as)
        });
    }

    fn close_file_event(
        &self,
        save_item: &MenuItem,
        actual_item: &MenuItem,
        current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    ) {
        let window = self.window.clone();
        let editor = self.content.source.buff.clone();
        let headerbar = self.header.container.clone();
        let save_item = save_item.clone();

        actual_item.connect_activate(move |_| {
            close_file(&window, &editor, &headerbar, &save_item, &current_file)
        });
    }

    fn quit_event(&self, quit_item: &MenuItem) {
        quit_item.connect_activate(move |_| {
            main_quit();
        });
    }

    /// Beállítja a mentés gombot az alapján, hogy megváltozott-e a tartalom.
    fn editor_changed(
        &self,
        current_file: Arc<RwLock<Option<ActiveMetadata>>>,
        save_item: &MenuItem,
    ) {
        let save_item = save_item.clone();
        self.content.source.buff.connect_changed(move |editor| {
            if let Some(source_code) = get_buffer(&editor) {
                if let Some(ref current_file) = *current_file.read().unwrap() {
                    let has_same_sum = current_file.is_same_as(&source_code.as_bytes());
                    save_item.set_sensitive(!has_same_sum);
                }
            }
        });
    }
}
