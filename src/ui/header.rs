use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub open: Button,
    pub save: Button,
    pub save_as: Button,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title("Rozsda IDE");
        container.set_show_close_button(true);

        let open = Button::new_with_mnemonic("Megnyitás");
        let save = Button::new_with_mnemonic("Mentés");
        let save_as = Button::new_with_mnemonic("Mentés Másként");

        container.pack_start(&open);
        container.pack_end(&save_as);
        container.pack_end(&save);

        Header {
            container,
            open,
            save,
            save_as,
        }
    }
}
