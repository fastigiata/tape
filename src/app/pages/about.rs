use eframe::egui;
use eframe::egui::{Align, Rect, Separator, Ui};
use eframe::egui::output::OpenUrl;
use crate::app::core::{TapeApp};
use crate::app::prefab;

const TAPE_DESC_ZH: &'static str = "Tape 是一个用于记录鼠标和/或键盘操作的应用，可以生成记录文件用于稍后播放。";
const TAPE_DESC_EN: &'static str = "Tape is an application for recording mouse and/or keyboard actions, which can generate a recording file for later playback.";
const TAPE_LICENSE_URL: &'static str = "https://github.com/lopo12123/tape/blob/master/LICENSE";

pub fn about_renderer(_app: &mut TapeApp, rect: Rect, ui: &mut Ui, _frame: &mut eframe::Frame) {
    ui.allocate_ui_at_rect(
        {
            let mut v = rect;
            v.min += egui::vec2(16.0, 16.0);
            v.max -= egui::vec2(16.0, 16.0);
            v
        },
        |ui| {
            ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                // ===== section: description (zh)
                ui.label(prefab::text_zh("说明", 24.0));
                ui.label(prefab::text_zh(TAPE_DESC_ZH, 16.0));

                // separator
                ui.add(Separator::default().spacing(16.0));

                // ===== section: license (zh)
                ui.label(prefab::text_zh("许可证", 24.0));
                ui.label(prefab::text_en("MIT. Copyright (c) 2023 lopo12123", 16.0));
                if ui.link(prefab::text_zh("点击此处查看详情", 16.0)).clicked() {
                    ui.ctx().output_mut(|o| {
                        o.open_url = Some(OpenUrl::same_tab(TAPE_LICENSE_URL));
                    });
                }

                // separator
                ui.add(Separator::default().spacing(16.0));

                // ===== section: description (en)
                ui.label(prefab::text_en("Description", 24.0));
                ui.label(prefab::text_en(TAPE_DESC_EN, 16.0));

                // separator
                ui.add(Separator::default().spacing(16.0));

                // ===== section: license (en)
                ui.label(prefab::text_en("License", 24.0));
                ui.label(prefab::text_en("MIT. Copyright (c) 2023 lopo12123", 16.0));
                if ui.link(prefab::text_en("Click here for details", 16.0)).clicked() {
                    ui.ctx().output_mut(|o| {
                        o.open_url = Some(OpenUrl::same_tab(TAPE_LICENSE_URL));
                    });
                }
            });
        },
    );
}