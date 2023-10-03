//! This file contains the preludes of the app. Includes the following:
//! - color selection
//! - font selection
//! - icon selection
use std::collections::HashMap;
use eframe::egui::{Color32, Context, FontData, FontDefinitions, FontFamily, Image, Vec2};

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
const FONT_ICONFONT: &'static [u8] = include_bytes!("../assets/fonts/iconfont.ttf");

#[derive(Debug)]
pub struct FontIcon(&'static str);

impl FontIcon {
    pub const CLOSE: FontIcon = FontIcon("\u{e613}");
    pub const DARK: FontIcon = FontIcon("\u{e821}");
    pub const LIGHT: FontIcon = FontIcon("\u{e825}");
    pub const SETTING: FontIcon = FontIcon("\u{e61d}");
    pub const MIN: FontIcon = FontIcon("\u{e617}");

    pub fn code(self) -> &'static str {
        self.0
    }
}

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

    // add the font -- iconfont
    fonts.font_data.insert(
        "iconfont".to_string(),
        FontData::from_static(FONT_ICONFONT),
    );
    fonts.families.insert(
        FontFamily::Name("iconfont".into()),
        vec!["iconfont".to_string()],
    );

    // set the fonts
    ctx.set_fonts(fonts);
}
// endregion

// region -- icon selection
const ICON_BACK: &'static str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M490.44791412 387.6313324V263.41716003c0-29.74033357-34.06620026-47.35279084-57.70397186-29.35409545-0.46348572 0.3862381-0.92697143 0.69522858-1.39045715 1.08146667L128.6973114 487.35800934c-16.22200012 13.51833344-16.22200012 35.68840027 0 49.28398132l302.65617371 252.21347809c0.46348572 0.3862381 0.92697143 0.7724762 1.39045715 1.08146667 23.63777161 17.99869538 57.70397187 0.30899048 57.70397186-29.35409545v-136.72828674c71.22230529-0.30899048 363.45005036 8.34274292 417.05989838 171.64421081 0-373.41499328-339.50328827-405.47275544-417.05989838-407.86743164z" fill="#FED843"/></svg>"##;
const ICON_CLOSE: &'static str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M512 577.61755591l-342.13921577 342.13921577-65.61755591-65.61755591L446.38244409 512 104.24322832 169.86078423l65.61755591-65.61755591L512 446.38244409l342.13921577-342.13921577 65.61755591 65.61755591L577.61755591 512l342.13921577 342.13921577-65.61755591 65.61755591L512 577.61755591z" fill="#EE1111"/></svg>"##;
const ICON_DARK: &'static str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M530.432 945.3568A434.176 434.176 0 0 1 491.52 78.6432a40.96 40.96 0 0 1 26.0096 70.8608 261.7344 261.7344 0 0 0-83.5584 192.3072 266.24 266.24 0 0 0 266.24 266.24 262.3488 262.3488 0 0 0 191.6928-82.944s0 1.024 0 0a40.96 40.96 0 0 1 70.656 24.576 434.176 434.176 0 0 1-432.128 395.6736z m0 0" fill="#8486F8"/></svg>"##;
const ICON_LIGHT: &'static str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M230.75 512a4.5 4.5 0 1 0 562.5 0 4.5 4.5 0 1 0-562.5 0z m286.5942-337.5c28.125 0 50.9058-22.7817 50.9058-50.9067V112.9067C568.25 84.7817 545.4683 62 517.3433 62s-50.9058 22.7817-50.9058 50.9067v10.6866c0 28.125 22.8375 50.9067 50.9067 50.9067zM205.1567 269.5625c19.8558 19.9125 52.0875 19.9125 71.9433 0 19.9125-19.8567 19.9125-52.0875 0-71.9442l-7.5375-7.5933c-19.8567-19.8567-52.0875-19.8567-71.9442 0-19.9125 19.8567-19.9125 52.0875 0 71.9442l7.5375 7.5933zM123.5942 455.75H112.9058C84.8375 455.75 62 478.5317 62 506.6567c0 28.125 22.7817 50.9058 50.85 50.85h10.6875c28.1817 0.0558 50.9625-22.7817 50.9625-50.85 0-28.125-22.7817-50.9067-50.9067-50.9067z m74.025 291.15l-7.5942 7.5375c-19.8567 19.9125-19.8567 52.1442 0 71.9442 19.8567 19.9125 52.0875 19.9125 71.9442 0l7.5933-7.5942c19.9125-19.8567 19.9125-52.0875 0-71.8875-19.8567-19.9692-52.0875-19.9692-71.9442 0z m309.0375 102.6c-28.125-0.0567-50.9067 22.725-50.9067 50.9067v10.6308c0 28.125 22.8375 50.9067 50.9067 50.85 28.125 0.0567 50.9058-22.725 50.85-50.85V900.35c0.0558-28.0692-22.7817-50.85-50.85-50.85z m312.1308-95.0625c-19.8567-19.8567-52.0875-19.8567-71.8875 0-19.9125 19.8567-19.9125 52.0875 0 72l7.5375 7.5375c19.9125 19.9125 52.1442 19.9125 71.9442 0 19.9125-19.8567 19.9125-52.0875 0-71.9442l-7.5942-7.5933z m92.3067-287.9442h-10.6875c-28.125 0-50.9067 22.7817-50.9067 50.85-0.0567 28.125 22.725 50.9067 50.9067 50.9067h10.6308c28.125 0 50.9067-22.7817 50.85-50.9067 0.1125-28.0683-22.6692-50.85-50.7942-50.85zM761.975 197.6192l-7.5375 7.5942C734.525 225.0692 734.525 257.3 754.3808 277.1c19.8567 19.9125 52.0875 19.9125 72 0l7.5375-7.5375c19.9125-19.9125 19.9125-52.1442 0-71.9442-19.8-19.8558-52.0308-19.8558-71.9433 0z" fill="#F4EA2A"/></svg>"##;
const ICON_MIN: &'static str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M937.691429 86.308571c17.554286 17.554286 17.554286 46.08 0 63.634286L708.169143 379.538286h153.892571a44.982857 44.982857 0 1 1 0 89.965714h-262.582857a44.982857 44.982857 0 0 1-44.982857-44.982857v-262.582857a44.982857 44.982857 0 1 1 90.038857 0v153.965714L874.057143 86.308571c17.554286-17.554286 46.08-17.554286 63.634286 0zM116.882286 599.552c0-24.868571 20.187429-45.056 45.056-45.056h262.582857c24.868571 0 44.982857 20.187429 44.982857 45.056v262.582857a44.982857 44.982857 0 1 1-90.038857 0V708.096L150.016 937.691429a44.982857 44.982857 0 0 1-63.634286-63.634286l229.449143-229.522286H161.938286a44.982857 44.982857 0 0 1-45.056-44.982857z" fill="#1296DB"/></svg>"##;
const ICON_TAPE: &'static str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M327.293585 328.047094c-67.719245-24.305509-107.075623 17.427321-107.24951 87.716227-0.193208 90.537057 63.642566 201.167698 142.31668 246.629434 60.899019 35.279698 116.581434 21.890415 130.221887-45.152604 0.193208-0.908075 0.270491-1.738868 0.444377-2.627623a156.111698 156.111698 0 0 1-14.046189-7.245283c-88.798189-51.315925-159.859925-180.262642-151.687245-279.320151zM618.012981 747.384755c-4.559698 12.751698-12.925585 29.425509-14.104151 31.454188l142.645132-82.750792c46.408453-27.12634 72.066415-84.48966 72.259623-161.521509 0.77283-243.827925-267.244679-566.948226-438.00151-467.94868l-142.645132 82.886038c43.471698 0.309132 88.353811 16.384 125.391699 37.733434 176.128 101.743094 317.980981 382.821434 254.454339 560.147321z" fill="#1296db"/><path d="M353.569811 204.993208c-54.291321-31.512151-123.749434-50.291925-178.272603-18.799095-46.640302 26.855849-72.665358 84.199849-72.877887 161.714717-0.386415 164.941283 116.079094 366.630642 259.419773 449.400755 47.451774 27.242264 92.275925 38.371019 131.226566 33.714717 44.437736-4.636981 83.388377-32.710038 104.17751-85.784151 58.155472-149.619925-54.098113-431.046038-243.673359-540.266264z m158.681359 416.478188c-14.934943 73.882566-80.567547 104.370717-159.898566 58.348679-84.393057-48.668981-152.82717-167.16317-152.421434-264.076075 0.193208-104.583245 78.944604-144.345358 163.33766-93.261283 101.743094 58.541887 170.177208 209.14717 148.98234 298.988679zM916.673208 860.391849l-206.384302-119.846641c2.028679-1.178566-165.250415 98.729057-191.024302 106.070943l219.619019 127.74883a9.872906 9.872906 0 0 0 9.911547 0l167.839396-96.893585a9.872906 9.872906 0 0 0 0.038642-17.079547z" fill="#1afa29"/></svg>"##;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum IconName {
    Back,
    Close,
    Dark,
    Light,
    Min,
    TAPE,
}

impl IconName {
    pub fn uri(&self) -> &'static str {
        match self {
            IconName::Back => "bytes://icon_back.svg",
            IconName::Min => "bytes://icon_min.svg",
            IconName::Close => "bytes://icon_close.svg",
            IconName::Dark => "bytes://icon_dark.svg",
            IconName::Light => "bytes://icon_light.svg",
            IconName::TAPE => "bytes://icon_tape.svg",
        }
    }
}

pub struct TapeIcon {
    images: HashMap<IconName, Image<'static>>,
}

impl TapeIcon {
    pub fn new() -> TapeIcon {
        TapeIcon { images: prepare_image() }
    }

    pub fn image(&self, name: IconName) -> Image {
        self.images.get(&name).expect("Icon not found").clone()
    }

    pub fn sized_image(&self, name: IconName, size: Vec2) -> Image {
        self.images.get(&name).expect("Icon not found").clone().fit_to_exact_size(size)
    }
}

fn prepare_image() -> HashMap<IconName, Image<'static>> {
    let mut images = HashMap::new();

    let image_raw = vec![
        (IconName::Back, ICON_BACK),
        (IconName::Min, ICON_MIN),
        (IconName::Close, ICON_CLOSE),
        (IconName::Dark, ICON_DARK),
        (IconName::Light, ICON_LIGHT),
        (IconName::TAPE, ICON_TAPE),
    ];

    for (name, raw) in image_raw {
        let uri = name.uri();
        images.insert(name, Image::from_bytes(uri, raw.as_bytes()));
    }

    images
}
// endregion