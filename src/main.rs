use crate::app::{run_tape_app};

mod act;
mod app;
mod canonicalize;
mod record;
mod utils;

fn main() {
    println!("Hello world!");

    // run the app
    run_tape_app();
}
