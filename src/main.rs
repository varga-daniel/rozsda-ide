extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate sourceview;
extern crate tiny_keccak;
extern crate pango;

pub mod cargo;
pub mod state;
pub mod ui;

use ui::App;

fn main() {
    // Helló!
    // Ez a Rozsda IDE-vel lett írva!
    println!("Hello, world!");

    App::new().connect_events().then_execute();

    // És ez is!
    println!("Kész!");
}
