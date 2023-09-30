use eframe::egui::{FontFamily, FontId, RichText};

pub fn text_en(text: impl Into<String>, size: f32) -> RichText {
    RichText::new(text)
        .font(FontId {
            size,
            family: FontFamily::Name("Mooli".into()),
        })
}

pub fn text_zh(text: impl Into<String>, size: f32) -> RichText {
    RichText::new(text)
        .font(FontId {
            size,
            family: FontFamily::Name("MaShanZheng".into()),
        })
}