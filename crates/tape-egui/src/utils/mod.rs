#[cfg(test)]
mod unit_test {
    use std::env;
    use tauri_winrt_notification::{IconCrop, Toast};

    fn notify() {
        let mut p = env::current_dir().unwrap();
        println!("Hello, world!");
        println!("{:?}", env::current_dir().unwrap());

        Toast::new(Toast::POWERSHELL_APP_ID)
            .icon(p.join("src/assets/tape.png").as_path(), IconCrop::Circular, "Tape")
            .title("Hello, world!")
            .text1("This is a notification")
            .show()
            .unwrap();
    }
}