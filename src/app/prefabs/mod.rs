use eframe::egui::{Button, FontFamily, FontId, RichText};

pub fn button(text: &str) -> Button {
    Button::new(
        RichText::new(text)
            .font(FontId::proportional(16.0))
    )
        .rounding(4.0)
        .frame(false)
}

pub fn text_en(text: &str, size: f32) -> RichText {
    RichText::new(text)
        .font(FontId {
            size,
            family: FontFamily::Name("Mooli".into()),
        })
}

pub fn text_zh(text: &str, size: f32) -> RichText {
    RichText::new(text)
        .font(FontId {
            size,
            family: FontFamily::Name("MaShanZheng".into()),
        })
}