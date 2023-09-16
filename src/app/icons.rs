use std::collections::HashMap;
use eframe::egui::TextureId;
use egui_extras::RetainedImage;

const ICON_MIN: &str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M512 958.72c-245.461333 0-444.458667-199.04-444.458667-444.501333C67.541333 268.8 266.538667 69.802667 512 69.802667s444.458667 198.954667 444.458667 444.416S757.461333 958.72 512 958.72zM289.792 469.76v88.917333h444.416v-88.874666H289.792z" fill="#FA923F"/></svg>"##;

const ICON_CLOSE: &str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M512 962A450 450 0 1 1 512 62a450 450 0 0 1 0 900zM330.90714248 387.54285752L455.3 512 330.90714248 636.45714248a40.05 40.05 0 1 0 56.63571504 56.63571504L512 568.7l124.45714248 124.39285752a40.05 40.05 0 0 0 56.63571504-56.63571504L568.7 512l124.39285752-124.45714248a40.05 40.05 0 1 0-56.63571504-56.63571504L512 455.3 387.54285752 330.90714248a40.05 40.05 0 0 0-56.63571504 56.63571504z" fill="#d81e06"/></svg>"##;

const ICON_DARK: &str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256">
    <path d="M507.1872 520.96m-450.816 0a450.816 450.816 0 1 0 901.632 0 450.816 450.816 0 1 0-901.632 0Z"
          fill="#8486F8"/>
    <path d="M503.9104 200.0896c-2.304 0-4.608 0-6.912 0.0512-11.4688 0.256-15.6672 15.4624-5.7344 21.248 66.3552 38.7072 109.568 112.7936 103.7312 196.4032-7.2192 103.6288-91.2384 187.4944-194.8672 194.4576-83.968 5.6832-158.208-38.0416-196.6592-104.96-5.7344-9.984-20.9408-5.7856-21.248 5.6832-0.1024 2.9696-0.1024 5.9392-0.1024 8.9088 0 182.784 152.3712 329.984 336.9984 321.4848 164.864-7.5776 298.7008-141.4144 306.2784-306.2784 8.448-184.576-138.7008-336.9984-321.4848-336.9984z"
          fill="#FFFFFF"/>
    <path d="M361.5744 476.16m-50.2272 0a50.2272 50.2272 0 1 0 100.4544 0 50.2272 50.2272 0 1 0-100.4544 0Z"
          fill="#FFFFFF"/>
</svg>"##;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum IconName {
    Min,
    Close,
    Dark,
}

pub struct TapeIcon {
    icons: HashMap<IconName, Option<TextureId>>,
}

impl TapeIcon {
    pub fn new(ctx: &eframe::egui::Context) -> TapeIcon {
        TapeIcon { icons: icon_loader(ctx) }
    }

    pub fn get(&self, name: IconName) -> Option<TextureId> {
        self.icons.get(&name).map_or(None, |v| v.clone())
    }

    pub fn count(&self) -> usize {
        self.icons.len()
    }

    pub fn valid_count(&self) -> usize {
        self.icons.iter().filter(|(_, v)| v.is_some()).count()
    }
}

fn icon_loader(ctx: &eframe::egui::Context) -> HashMap<IconName, Option<TextureId>> {
    let mut icons: HashMap<IconName, Option<TextureId>> = HashMap::new();

    if let Ok(img) = RetainedImage::from_svg_str("icon_min", ICON_MIN) {
        icons.insert(IconName::Min, Some(img.texture_id(ctx)));
    }
    if let Ok(img) = RetainedImage::from_svg_str("icon_close", ICON_CLOSE) {
        icons.insert(IconName::Close, Some(img.texture_id(ctx)));
    }
    if let Ok(img) = RetainedImage::from_svg_str("icon_dark", ICON_DARK) {
        icons.insert(IconName::Dark, Some(img.texture_id(ctx)));
    }
    icons
}