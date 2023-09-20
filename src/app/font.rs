use eframe::egui::{Context, FontData, FontDefinitions, FontFamily};

const FONT_MA_SHAN_ZHENG: &'static [u8] = include_bytes!("../assets/fonts/MaShanZheng-Regular.ttf");

pub fn prepare_font(ctx: &Context) {
    // default fonts (from egui)
    let mut fonts = FontDefinitions::default();

    // add the font
    fonts.font_data.insert(
        "MaShanZheng".to_string(),
        FontData::from_static(FONT_MA_SHAN_ZHENG),
    );

    // put the font in the first place
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "MaShanZheng".to_string());

    // set the fonts
    ctx.set_fonts(fonts);
}