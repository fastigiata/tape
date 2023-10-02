//! This file contains the notifier of the app.

use tauri_winrt_notification::{Duration, Toast};

const APP_ID: &'static str = "tray_clock";
// const APP_ID: &'static str = "tape";

pub struct Notifier {}

impl Notifier {
    pub fn notify(detail: &str) {
        Toast::new(APP_ID)
            .title("Tape")
            .text1(detail)
            .duration(Duration::Short)
            .show()
            .expect("Failed to show toast");
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn ttt() {
        Notifier::notify("hello");
    }
}