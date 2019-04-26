use gtk::*;

pub struct CargoMenu {
    pub cargo_menu_item: MenuItem,
    pub cargo_menu: Menu,
    pub new_lib_item: MenuItem,
    pub new_bin_item: MenuItem,
    pub open_item: MenuItem,
    pub close_item: MenuItem,
    pub build_item: MenuItem,
    pub check_item: MenuItem,
    pub test_item: MenuItem,
    pub clean_item: MenuItem,
    pub run_item: MenuItem,
}

impl CargoMenu {
    pub fn new() -> CargoMenu {
        let cargo_menu = Menu::new();
        let cargo_menu_item = MenuItem::new_with_label("Cargo");
        cargo_menu_item.set_submenu(&cargo_menu);

        let new_lib_item = MenuItem::new_with_label("Új könyvtár");
        let new_bin_item = MenuItem::new_with_label("Új bináris");
        let open_item = MenuItem::new_with_label("Megnyitás");
        let close_item = MenuItem::new_with_label("Bezárás");
        let build_item = MenuItem::new_with_label("Fordítás");
        let check_item = MenuItem::new_with_label("Ellenőrzés");
        let test_item = MenuItem::new_with_label("Tesztek futtatása");
        let clean_item = MenuItem::new_with_label("Takarítás");
        let run_item = MenuItem::new_with_label("Futtatás");

        cargo_menu.append(&new_lib_item);
        cargo_menu.append(&new_bin_item);
        cargo_menu.append(&open_item);
        cargo_menu.append(&close_item);
        cargo_menu.append(&SeparatorMenuItem::new());
        cargo_menu.append(&build_item);
        cargo_menu.append(&check_item);
        cargo_menu.append(&test_item);
        cargo_menu.append(&clean_item);
        cargo_menu.append(&run_item);

        CargoMenu {
            cargo_menu_item,
            cargo_menu,
            new_lib_item,
            new_bin_item,
            open_item,
            close_item,
            build_item,
            check_item,
            test_item,
            clean_item,
            run_item,
        }
    }
}
