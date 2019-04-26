pub mod cargo_menu;
pub mod file_menu;

use crate::state::*;
use cargo_menu::*;
use file_menu::*;
use gtk::*;
use std::sync::{Arc, RwLock};

pub struct Header {
    pub container: HeaderBar,
    pub file_menu: FileMenu,
    pub cargo_menu: CargoMenu,

    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    current_project: Arc<RwLock<Option<ProjectMetadata>>>,
}

impl Header {
    pub fn new(
        current_file: Arc<RwLock<Option<ActiveMetadata>>>,
        current_project: Arc<RwLock<Option<ProjectMetadata>>>,
    ) -> Header {
        let container = HeaderBar::new();

        container.set_title("Rozsda IDE");
        container.set_show_close_button(true);

        let file_menu = FileMenu::new();
        let cargo_menu = CargoMenu::new();

        container.pack_start(&file_menu.file_menu_bar);
        container.pack_start(&cargo_menu.cargo_menu_bar);

        Header {
            container,
            file_menu,
            cargo_menu,
            current_file,
            current_project,
        }
    }

    pub fn update_titles(&self, same_sum: bool) {
        let mut final_title = String::from("Rozsda IDE");
        let mut final_subtitle = String::new();
        self.container.set_subtitle("");

        if let Some(ref current_file) = *self.current_file.read().unwrap() {
            final_title = String::from(current_file.get_path().to_string_lossy());
            final_subtitle = String::from(current_file.get_dir().unwrap().to_string_lossy());
        }

        if !same_sum {
            final_title.push_str("*");
        }

        if let Some(ref current_project) = *self.current_project.read().unwrap() {
            final_title.push_str(" - ");
            final_title.push_str(&current_project.get_path().to_string_lossy());
        }

        self.container.set_title(final_title.as_str());
        self.container.set_subtitle(final_subtitle.as_str());
    }
}
