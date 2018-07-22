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