use std::collections::HashMap;
use egui_extras::RetainedImage;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum IconName {
    Min,
    Close,
    Dark,
    Light,
}

pub struct TapeIcon {
    icons: HashMap<IconName, RetainedImage>,
}

impl TapeIcon {
    pub fn new() -> TapeIcon {
        TapeIcon { icons: icon_loader() }
    }

    pub fn get(&self, name: IconName) -> &RetainedImage {
        self.icons.get(&name).expect("Icon not found")
    }

    pub fn try_get(&self, name: IconName) -> Option<&RetainedImage> {
        self.icons.get(&name)
    }

    pub fn count(&self) -> usize {
        self.icons.len()
    }
}

fn icon_loader() -> HashMap<IconName, RetainedImage> {
    let mut icons: HashMap<IconName, RetainedImage> = HashMap::new();


    if let Ok(img) = RetainedImage::from_image_bytes("icon_close", include_bytes!("../assets/close.png")) {
        icons.insert(IconName::Close, img);
    }
    if let Ok(img) = RetainedImage::from_image_bytes("icon_light", include_bytes!("../assets/light.png")) {
        icons.insert(IconName::Light, img);
    }
    if let Ok(img) = RetainedImage::from_image_bytes("icon_dark", include_bytes!("../assets/dark.png")) {
        icons.insert(IconName::Dark, img);
    }
    if let Ok(img) = RetainedImage::from_image_bytes("icon_min", include_bytes!("../assets/min.png")) {
        icons.insert(IconName::Min, img);
    }

    icons
}