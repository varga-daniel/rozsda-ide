pub mod cargo_menu;
pub mod file_menu;

use cargo_menu::*;
use file_menu::*;
use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub file_menu: FileMenu,
    pub cargo_menu: CargoMenu,
}

impl Header {
    pub fn new() -> Header {
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
        }
    }
}
