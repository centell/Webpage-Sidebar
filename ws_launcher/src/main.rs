extern crate yew;
use yew::prelude::App;

extern crate ws_launcher;
use ws_launcher::MainModel;

fn main() {
   yew::initialize();
   let app = App::<MainModel>::new();
   app.mount_to_body();
   yew::run_loop();
}
