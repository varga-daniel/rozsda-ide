use gtk::*;

pub struct FileMenu {
    pub file_menu_bar: MenuBar,
    pub file_menu: Menu,
    pub new_file_item: MenuItem,
    pub open_item: MenuItem,
    pub save_item: MenuItem,
    pub save_as_item: MenuItem,
    pub close_file_item: MenuItem,
    pub quit_item: MenuItem,
}

impl FileMenu {
    pub fn new() -> FileMenu {
        let file_menu_bar = MenuBar::new();

        let file_menu = Menu::new();
        let file_menu_item = MenuItem::new_with_label("Fájl");
        file_menu_item.set_submenu(&file_menu);

        let new_file_item = MenuItem::new_with_label("Új");
        let open_item = MenuItem::new_with_label("Megnyitás");
        let save_item = MenuItem::new_with_label("Mentés");
        let save_as_item = MenuItem::new_with_label("Mentés Másként");
        let close_file_item = MenuItem::new_with_label("Bezárás");
        let quit_item = MenuItem::new_with_label("Kilépés");

        file_menu.append(&new_file_item);
        file_menu.append(&SeparatorMenuItem::new());
        file_menu.append(&open_item);
        file_menu.append(&save_item);
        file_menu.append(&save_as_item);
        file_menu.append(&close_file_item);
        file_menu.append(&SeparatorMenuItem::new());
        file_menu.append(&quit_item);

        file_menu_bar.append(&file_menu_item);

        FileMenu {
            file_menu_bar,
            file_menu,
            new_file_item,
            open_item,
            save_item,
            save_as_item,
            close_file_item,
            quit_item,
        }
    }
}
