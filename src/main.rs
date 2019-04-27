pub mod cargo;
pub mod state;
pub mod ui;

use ui::App;

fn main() {
    App::new().connect_events().then_execute();
}
