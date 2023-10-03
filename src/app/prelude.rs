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
const ICON_TAPE: &'static str = r##"<svg viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="256" height="256"><path d="M314.979824 315.78356693c-72.23386133-25.92587627-114.21399787 18.5891424-114.39947733 93.56397547-0.20608853 96.5728608 67.88540373 214.57887787 151.80445866 263.07139627 64.9589536 37.63167787 124.3535296 23.349776 138.90334614-48.1627776 0.20608853-0.96861333 0.28852373-1.85479253 0.47400213-2.80279787a166.51914453 166.51914453 0 0 1-14.9826016-7.72830187c-94.71806827-54.73698667-170.51725333-192.28015147-161.799728-297.9414944zM625.08051307 763.077072c-4.86367787 13.6018112-13.78729067 31.3872096-15.04442774 33.55113387l152.15480747-88.26751147c49.50234987-28.93476267 76.87084267-90.122304 77.0769312-172.2896096 0.824352-260.08312-285.06099093-604.7447744-467.20161067-499.14525867l-152.15480746 88.41177387c46.3698112 0.3297408 94.24406507 17.47626667 133.7511456 40.24899627 187.86986667 108.52596693 339.17971307 408.34286293 271.4179616 597.49047573z" fill="#1296db"/><path d="M343.0077984 184.52608853c-57.9107424-33.61296107-131.99939627-53.64472-190.1574432-20.052368-49.74965547 28.64623893-77.5097152 89.81317227-77.7364128 172.49569814-0.412176 175.93736853 123.81770027 391.0726848 276.71442453 479.36080533 50.6152256 29.05841493 98.42765333 40.92908693 139.97500374 35.9623648 47.40025173-4.94611307 88.94760213-34.8907072 111.12267733-91.5030944 62.03250347-159.59458667-57.70465387-459.78244053-259.9182496-576.28401493z m169.26011627 444.24340054c-15.93060587 78.8080704-85.9387168 111.3287648-170.5584704 62.23859093-90.0192608-51.91357973-163.015648-178.30738133-162.58286294-281.68114667 0.20608853-111.55546133 84.2075776-153.96838187 174.22683734-99.47870186 108.52596693 62.44467947 181.5223552 223.09031467 158.914496 318.9212576zM943.65142187 883.61797227l-220.14325547-127.83641707c2.16392427-1.25713707-176.26710933 105.31099413-203.75925547 113.1423392l234.26028694 136.26541867a10.53109973 10.53109973 0 0 0 10.5723168 0l179.02868906-103.35315734a10.53109973 10.53109973 0 0 0 0.04121814-18.21818346z" fill="#1afa29"/></svg>"##;

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