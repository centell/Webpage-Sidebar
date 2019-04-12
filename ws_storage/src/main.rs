extern crate ws_storage;
use ws_storage::*;

fn main() {
    stdweb::initialize();

    let storage = Storage::new();
    storage.bind_js();

    stdweb::event_loop();
}