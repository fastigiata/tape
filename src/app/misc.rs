use std::collections::HashMap;
use eframe::egui::{Color32, Context, FontData, FontDefinitions, FontFamily, TextureId};
use egui_extras::RetainedImage;

// region -- color selection
const YELLOW: Color32 = Color32::from_rgb(254, 216, 67);

pub struct TapeColor {}

impl TapeColor {
    pub const YELLOW: Color32 = YELLOW;
}

// endregion

// region -- font selection
const FONT_MASHANZHENG: &'static [u8] = include_bytes!("../assets/fonts/MaShanZheng-Regular.ttf");
const FONT_MOOLI: &'static [u8] = include_bytes!("../assets/fonts/Mooli-Regular.ttf");

pub fn prepare_font(ctx: &Context) {
    // default fonts (from egui)
    let mut fonts = FontDefinitions::default();

    // add the font -- MaShanZheng
    fonts.font_data.insert(
        "MaShanZheng".to_string(),
        FontData::from_static(FONT_MASHANZHENG),
    );
    fonts.families.insert(
        FontFamily::Name("MaShanZheng".into()),
        vec!["MaShanZheng".to_string()],
    );

    // add the font -- Mooli
    fonts.font_data.insert(
        "Mooli".to_string(),
        FontData::from_static(FONT_MOOLI),
    );
    fonts.families.insert(
        FontFamily::Name("Mooli".into()),
        vec!["Mooli".to_string()],
    );

    // set the fonts
    ctx.set_fonts(fonts);
}
// endregion

// region -- icon selection
const ICON_MIN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M22 10H2C0.89543 10 0 10.8954 0 12C0 13.1046 0.89543 14 2 14H22C23.1046 14 24 13.1046 24 12C24 10.8954 23.1046 10 22 10Z" fill="#FA923F"/></svg>"##;
const ICON_CLOSE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M14.3014 12.0272L23.5069 2.88435C24.1644 2.23129 24.1644 1.14286 23.5069 0.489796C22.8493 -0.163265 21.7534 -0.163265 21.0959 0.489796L12.1096 9.85034L2.90411 0.707483C2.24658 0.0544218 1.15069 0.0544218 0.493151 0.707483C-0.164384 1.36054 -0.164384 2.44898 0.493151 3.10204L9.69863 12.2449L0.493151 21.1701C-0.164384 21.8231 -0.164384 22.9116 0.493151 23.5646C0.712329 23.7823 1.15069 24 1.58904 24C2.0274 24 2.46575 23.7823 2.68493 23.5646L11.8904 14.4218L21.3151 23.5646C21.5342 23.7823 21.9726 24 22.411 24C22.8493 24 23.2877 23.7823 23.5069 23.5646C24.1644 22.9116 24.1644 21.8231 23.5069 21.1701L14.3014 12.0272Z" fill="#FF0000"/></svg>"##;
const ICON_DARK: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="25" height="25" viewBox="0 0 25 25" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M12.0947 0.467119C12.1806 0.46521 12.2665 0.46521 12.3525 0.46521C19.1682 0.46521 24.6552 6.14884 24.3402 13.0314C24.0576 19.179 19.067 24.1696 12.9195 24.4521C6.03499 24.7691 0.353271 19.2802 0.353271 12.4644C0.353271 12.3537 0.353271 12.2429 0.35709 12.1322C0.368545 11.7045 0.935571 11.548 1.1494 11.9203C2.58319 14.4156 5.3515 16.046 8.48256 15.8341C12.3467 15.5745 15.4797 12.4472 15.7489 8.58304C15.9665 5.46535 14.3552 2.70277 11.8809 1.25943C11.5105 1.04369 11.6671 0.476665 12.0947 0.467119ZM5.72061 12.0839C5.36937 11.7326 5.17205 11.2562 5.17205 10.7595C5.17205 10.2628 5.36937 9.78641 5.72061 9.43517C6.07185 9.08393 6.54823 8.88661 7.04495 8.88661C7.54168 8.88661 8.01806 9.08393 8.36929 9.43517C8.72053 9.78641 8.91786 10.2628 8.91786 10.7595C8.91786 11.2562 8.72053 11.7326 8.36929 12.0839C8.01806 12.4351 7.54168 12.6324 7.04495 12.6324C6.54823 12.6324 6.07185 12.4351 5.72061 12.0839Z" fill="#8486F8"/></svg>"##;
const ICON_LIGHT: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4.39886 2.62286L6.40114 4.63886L4.8 6.25028L2.80114 4.236L4.39886 2.62114V2.62286ZM0 10.8891H3.19886V13.1074H0V10.8891ZM10.9989 0H13.2V3.22628H10.9989V0ZM21.1989 4.23429L19.6011 2.62286L17.5989 4.63886L19.2 6.25028L21.1989 4.236V4.23429ZM17.5989 19.3611L19.6011 21.3771L21.1989 19.7657L19.2 17.748L17.5989 19.3594V19.3611ZM20.8011 10.8909H24V13.1091H20.8011V10.8909ZM12 5.44457C8.4 5.44457 5.4 8.47029 5.4 12.1011C5.4 15.7303 8.4 18.756 12 18.756C15.6 18.756 18.6 15.7303 18.6 12.1011C18.6 8.26971 15.6 5.44457 12 5.44457ZM10.9989 20.772H13.2V24H10.9989V20.772ZM2.80114 19.764L4.39886 21.3771L6.40114 19.3611L4.8 17.748L2.80114 19.764Z" fill="#6FD6BF"/></svg>"##;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum IconName {
    Min,
    Close,
    Dark,
    Light,
}

impl IconName {
    pub fn debug_name(&self) -> &str {
        match self {
            IconName::Min => "icon_min",
            IconName::Close => "icon_close",
            IconName::Dark => "icon_dark",
            IconName::Light => "icon_light",
        }
    }
}

pub struct TapeIcon {
    icons: HashMap<IconName, RetainedImage>,
}

impl TapeIcon {
    pub fn new() -> TapeIcon {
        TapeIcon { icons: prepare_icon() }
    }

    pub fn get(&self, name: IconName) -> &RetainedImage {
        self.icons.get(&name).expect("Icon not found")
    }

    pub fn get_texture_id(&self, name: IconName, ctx: &Context) -> TextureId {
        self.icons.get(&name).expect("Icon not found").texture_id(ctx)
    }

    pub fn try_get(&self, name: IconName) -> Option<&RetainedImage> {
        self.icons.get(&name)
    }

    pub fn count(&self) -> usize {
        self.icons.len()
    }
}

fn prepare_icon() -> HashMap<IconName, RetainedImage> {
    let mut icons: HashMap<IconName, RetainedImage> = HashMap::new();

    let icon_raw = vec![
        (IconName::Min, ICON_MIN),
        (IconName::Close, ICON_CLOSE),
        (IconName::Dark, ICON_DARK),
        (IconName::Light, ICON_LIGHT),
    ];

    for (name, raw) in icon_raw {
        if let Ok(img) = RetainedImage::from_svg_str(name.debug_name(), raw) {
            icons.insert(name, img);
        }
    }

    icons
}
// endregion