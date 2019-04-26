use super::misc::*;
use super::save::save;
use super::{Content, Header};
use crate::state::*;
use gdk::enums::key;
use gdk::ModifierType;
use gtk;
use gtk::*;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

pub struct App {
    pub window: Window,
    pub header: Arc<Header>,
    pub content: Content,

    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    current_project: Arc<RwLock<Option<ProjectMetadata>>>,
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

        let current_file = Arc::new(RwLock::new(None));
        let current_project = Arc::new(RwLock::new(None));

        let window = Window::new(WindowType::Toplevel);
        let in_header = Header::new(current_file.clone(), current_project.clone());
        let header = Arc::new(in_header);
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
            current_file,
            current_project,
        }
    }

    /// Ez hozza létre a `ConnectedApp` struktúrát, amivel aztán tényleg dolgozunk.
    pub fn connect_events(self) -> ConnectedApp {
        // A teljes képernyősséget viszont egyszerűen csak egy bool-ként tároljuk.
        let fullscreen = Arc::new(AtomicBool::new(false));

        let save = &self.header.file_menu.save_item;
        let save_as = &self.header.file_menu.save_as_item;
        let close = &self.header.file_menu.close_file_item;
        let quit = &self.header.file_menu.quit_item;

        self.editor_changed(&save);
        self.open_event();
        self.save_event(&save, &save, false);
        self.save_event(&save, &save_as, true);
        self.close_file_event(&save, &close);
        self.close_file_event(&save, &quit);

        self.open_project_event();
        self.close_project_event();
        self.create_project_event();

        self.cargo_build_event();
        self.cargo_check_event();
        self.cargo_clean_event();
        self.cargo_run_event();
        self.cargo_test_event();

        self.quit_event(&quit);

        self.key_events(fullscreen);

        ConnectedApp(self)
    }

    /// Hozzákapcsol megadott billentyűkombinációkhoz / billentyűkhöz parancsokat a GDK
    /// által adott lehetőségekkel.
    fn key_events(&self, fullscreen: Arc<AtomicBool>) {
        // Bár kellenek nekünk a referenciák, nem akarunk velük mit csinálni, ezért klónozunk.
        let editor = self.content.source.buff.clone();
        let current_file = self.current_file.clone();
        let header = self.header.clone();
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
                        save(&editor, &save_item, &current_file, false);
                        header.update_titles(true);
                    }
                    // Megnyitás a Ctrl+O-val.
                    key if key == 'o' as u32
                        && eventkey.get_state().contains(ModifierType::CONTROL_MASK) =>
                    {
                        open_file(&editor, &current_file);
                        header.update_titles(true);
                    }
                    // Fájl bezárása Ctrl+W-val.
                    key if key == 'w' as u32
                        && eventkey.get_state().contains(ModifierType::CONTROL_MASK) =>
                    {
                        close_file(&window, &editor, &save_item, &current_file);
                        header.update_titles(true);
                    }
                    // Kilépés Ctrl+Q-val.
                    key if key == 'q' as u32
                        && eventkey.get_state().contains(ModifierType::CONTROL_MASK) =>
                    {
                        close_file(&window, &editor, &save_item, &current_file);
                        header.update_titles(true);
                        main_quit();
                    }
                    // Semmi egyébként.
                    _ => (),
                }
                Inhibit(false)
            });
    }

    /// A Megnyitás gomb beállítása fájlok megnyitására.
    fn open_event(&self) {
        let editor = self.content.source.buff.clone();
        let current_file = self.current_file.clone();
        let header = self.header.clone();

        self.header.file_menu.open_item.connect_activate(move |_| {
            open_file(&editor, &current_file);
            header.update_titles(true);
        });
    }

    /// A save funkció összekötése a megadott gombbal.
    fn save_event(&self, save_item: &MenuItem, actual_item: &MenuItem, save_as: bool) {
        let editor = self.content.source.buff.clone();
        let save_item = save_item.clone();
        let current_file = self.current_file.clone();
        let header = self.header.clone();

        actual_item.connect_activate(move |_| {
            save(&editor, &save_item, &current_file, save_as);
            header.update_titles(true);
        });
    }

    fn close_file_event(&self, save_item: &MenuItem, actual_item: &MenuItem) {
        let window = self.window.clone();
        let editor = self.content.source.buff.clone();
        let save_item = save_item.clone();
        let current_file = self.current_file.clone();
        let header = self.header.clone();

        actual_item.connect_activate(move |_| {
            close_file(&window, &editor, &save_item, &current_file);
            header.update_titles(true);
        });
    }

    fn quit_event(&self, quit_item: &MenuItem) {
        quit_item.connect_activate(move |_| {
            main_quit();
        });
    }

    fn open_project_event(&self) {
        let window = self.window.clone();
        let header = self.header.clone();
        let current_project = self.current_project.clone();

        self.header.cargo_menu.open_item.connect_activate(move |_| {
            open_project(&window, &current_project);
            header.update_titles(true);
        });
    }

    fn close_project_event(&self) {
        let current_project = self.current_project.clone();
        let header = self.header.clone();

        self.header
            .cargo_menu
            .close_item
            .connect_activate(move |_| {
                *current_project.write().unwrap() = None;
                header.update_titles(true);
            });
    }

    fn create_project_event(&self) {
        let window = self.window.clone();
        let current_project = self.current_project.clone();
        let header = self.header.clone();

        self.header
            .cargo_menu
            .new_lib_item
            .connect_activate(move |_| {
                create_project(&window, &current_project, false);
                header.update_titles(true);
            });

        let window = self.window.clone();
        let current_project = self.current_project.clone();
        let header = self.header.clone();

        self.header
            .cargo_menu
            .new_bin_item
            .connect_activate(move |_| {
                create_project(&window, &current_project, true);
                header.update_titles(true);
            });
    }

    fn cargo_build_event(&self) {
        let window = self.window.clone();
        let current_project = self.current_project.clone();

        self.header
            .cargo_menu
            .build_item
            .connect_activate(move |_| {
                perform_cargo_action(&window, &current_project, CargoAction::Build);
            });
    }

    fn cargo_check_event(&self) {
        let window = self.window.clone();
        let current_project = self.current_project.clone();

        self.header
            .cargo_menu
            .check_item
            .connect_activate(move |_| {
                perform_cargo_action(&window, &current_project, CargoAction::Check);
            });
    }

    fn cargo_test_event(&self) {
        let window = self.window.clone();
        let current_project = self.current_project.clone();

        self.header.cargo_menu.test_item.connect_activate(move |_| {
            perform_cargo_action(&window, &current_project, CargoAction::Test);
        });
    }

    fn cargo_clean_event(&self) {
        let window = self.window.clone();
        let current_project = self.current_project.clone();

        self.header
            .cargo_menu
            .clean_item
            .connect_activate(move |_| {
                perform_cargo_action(&window, &current_project, CargoAction::Clean);
            });
    }

    fn cargo_run_event(&self) {
        let window = self.window.clone();
        let current_project = self.current_project.clone();

        self.header.cargo_menu.run_item.connect_activate(move |_| {
            perform_cargo_action(&window, &current_project, CargoAction::Run);
        });
    }

    /// Beállítja a mentés gombot az alapján, hogy megváltozott-e a tartalom.
    fn editor_changed(&self, save_item: &MenuItem) {
        let save_item = save_item.clone();
        let current_file = self.current_file.clone();
        let header = self.header.clone();
        self.content.source.buff.connect_changed(move |editor| {
            if let Some(source_code) = get_buffer(&editor) {
                if let Some(ref current_file) = *current_file.read().unwrap() {
                    let has_same_sum = current_file.is_same_as(&source_code.as_bytes());
                    save_item.set_sensitive(!has_same_sum);
                    header.update_titles(has_same_sum);
                }

                if source_code.len() > 0 {
                    header.update_titles(false);
                } else {
                    header.update_titles(true);
                }
            }
        });
    }
}
