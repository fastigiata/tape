//! This file contains the prefabs of the app. Includes the following:
//! - Text
use eframe::egui::{FontFamily, FontId, RichText};
use crate::app::prelude::FontIcon;

#[deprecated(note = "not implemented")]
pub fn font_icon(icon: FontIcon, size: f32) -> RichText {
    RichText::new(icon.code()).font(FontId { size, family: FontFamily::Name("iconfont".into()) })
}

pub fn text_en(text: impl Into<String>, size: f32) -> RichText {
    RichText::new(text).font(FontId { size, family: FontFamily::Name("Mooli".into()) })
}

pub fn text_zh(text: impl Into<String>, size: f32) -> RichText {
    RichText::new(text).font(FontId { size, family: FontFamily::Name("MaShanZheng".into()) })
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn tt() {
        println!("\u{e613}");
    }
}