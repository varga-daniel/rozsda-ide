use gtk::*;

pub struct CargoMenu {
    pub cargo_menu_bar: MenuBar,
    pub cargo_menu: Menu,
    pub new_lib_init_item: MenuItem,
    pub new_lib_new_item: MenuItem,
    pub new_bin_init_item: MenuItem,
    pub new_bin_new_item: MenuItem,
    pub open_item: MenuItem,
    pub build_item: MenuItem,
    pub check_item: MenuItem,
    pub test_item: MenuItem,
    pub clean_item: MenuItem,
    pub run_item: MenuItem,
}

impl CargoMenu {
    pub fn new() -> CargoMenu {
        let cargo_menu_bar = MenuBar::new();

        let cargo_menu = Menu::new();
        let cargo_menu_item = MenuItem::new_with_label("Cargo");
        cargo_menu_item.set_submenu(&cargo_menu);

        let new_lib_menu = Menu::new();
        let new_lib_menu_item = MenuItem::new_with_label("Új láda");
        new_lib_menu_item.set_submenu(&new_lib_menu);

        let new_lib_init_item = MenuItem::new_with_label("Létező mappában");
        let new_lib_new_item = MenuItem::new_with_label("Új mappában");

        new_lib_menu.append(&new_lib_init_item);
        new_lib_menu.append(&new_lib_new_item);

        let new_bin_menu = Menu::new();
        let new_bin_menu_item = MenuItem::new_with_label("Új bináris");
        new_bin_menu_item.set_submenu(&new_bin_menu);

        let new_bin_init_item = MenuItem::new_with_label("Létező mappában");
        let new_bin_new_item = MenuItem::new_with_label("Új mappában");

        new_bin_menu.append(&new_bin_init_item);
        new_bin_menu.append(&new_bin_new_item);

        let open_item = MenuItem::new_with_label("Megnyitás");
        let build_item = MenuItem::new_with_label("Fordítás");
        let check_item = MenuItem::new_with_label("Ellenőrzés");
        let test_item = MenuItem::new_with_label("Tesztek futtatása");
        let clean_item = MenuItem::new_with_label("Takarítás");
        let run_item = MenuItem::new_with_label("Futtatás");

        cargo_menu.append(&new_lib_menu_item);
        cargo_menu.append(&new_bin_menu_item);
        cargo_menu.append(&open_item);
        cargo_menu.append(&SeparatorMenuItem::new());
        cargo_menu.append(&build_item);
        cargo_menu.append(&check_item);
        cargo_menu.append(&test_item);
        cargo_menu.append(&clean_item);
        cargo_menu.append(&run_item);

        cargo_menu_bar.append(&cargo_menu_item);

        CargoMenu {
            cargo_menu_bar,
            cargo_menu,
            new_lib_init_item,
            new_lib_new_item,
            new_bin_init_item,
            new_bin_new_item,
            open_item,
            build_item,
            check_item,
            test_item,
            clean_item,
            run_item,
        }
    }
}
