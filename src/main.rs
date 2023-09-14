use std::path::Path;
use tauri_winrt_notification::{IconCrop, Toast};

mod act;
mod app;
mod canonicalize;
mod record;

fn main() {
    println!("Hello, world!");

    // Toast::new(Toast::POWERSHELL_APP_ID)
    //     .hero(Path::new(r"D:\GitProject\tape\idea.png"), "Tape")
    //     .title("Hello, world!")
    //     .text1("This is a notification")
    //     .icon(Path::new(r"D:\GitProject\tape\idea.png"), IconCrop::Square, "Tape")
    //     .show()
    //     .unwrap();
}
