extern crate gio;
extern crate gtk;

/// A view modulban lévő ablakokat látja a felhasználó.
#[cfg(feature = "gtk_3_18")]
mod view {
    use gio;
    use gtk;

    use gio::prelude::*;
    use gtk::prelude::*;

    use std::env::args;

    macro_rules! clone {
        (@param _) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );
    }

    // CSAK DEBUG
    // TODO: Egyelőre összeomlasztja a programot.
    // Valahogy kijavítani.
    // Lehetséges hiba: mikor hívjuk meg.
    // Jó alternatíva lenne a GtkSourceView, de az csak GTK 3.20-ban vagy hogy van.
    pub fn detect_world(buffer: &gtk::TextBuffer) {
        let start = buffer.get_start_iter();
        let end = buffer.get_end_iter();

        let found = start.forward_search("világ", gtk::TextSearchFlags::all(), &end);

        if found.is_some() {
            let _found = found.unwrap();
            buffer.apply_tag(&buffer.get_tag_table().unwrap().lookup("bold").unwrap(), &_found.0, &_found.1);
            detect_world(buffer);
        }
    }

    pub fn build_ui(application: &gtk::Application) {
        let glade_src = include_str!("glade/main.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let window: gtk::Window = builder.get_object("MainWindow").unwrap();

        window.set_application(application);
        window.connect_delete_event(clone!(window => move |_, _| {
            window.destroy();
            Inhibit(false)
        }));

        let texteditor: gtk::TextView = builder.get_object("MainTextView").unwrap();
        let textbuffer: gtk::TextBuffer = texteditor.get_buffer().unwrap();
        
        textbuffer.set_text("Helló világ!");

        let bold_tag: gtk::TextTag = gtk::TextTag::new("bold");
        bold_tag.set_property_weight(700);
        textbuffer.get_tag_table().unwrap().add(&bold_tag);

        textbuffer.connect_end_user_action(clone!(textbuffer => move |_| {
            detect_world(&textbuffer);
        }));

        window.show_all();
    }

    /// Ez a metódus elindítja az applikációt
    pub fn main() {
        let application = gtk::Application::new("varga_daniel.rozsda_ide",
                                                gio::ApplicationFlags::empty())
                                           .expect("Az inicializáció meghiúsult!");

        application.connect_startup(move |app| {
            build_ui(app);
        });
        application.connect_activate(|_| {});

        application.run(&args().collect::<Vec<_>>());
    }
}

#[cfg(feature = "gtk_3_18")]
fn main() {
    // Meghívjuk a főablakot.
    view::main();
}

#[cfg(not(feature = "gtk_3_18"))]
fn main() {
    println!("Ehhez a programhoz GTK 3.18-as verzió szükségeltetik.");
    println!("Kérem, fordítsa újra a programot a --features gtk_3_18 kapcsolóval.");
}