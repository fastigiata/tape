use eframe::egui;
use eframe::egui::{Rect, ScrollArea, Separator, Ui};
use crate::app::core::{TapeApp};
use crate::app::component;

const TAPE_ABOUT: &'static str = "Tape is an application for recording mouse and/or keyboard actions, which can generate a recording file for later playback.";
const TAPE_LICENSE: &'static str = r##"Copyright (c) 2018-2021 lopo <lopo@zju.edu.cn>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."##;

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
            ScrollArea::vertical().show(ui, |ui| {
                // ===== section: description
                ui.label(component::text_en("About Tape", 18.0));
                ui.add_space(6.0);
                ui.label(component::text_en(TAPE_ABOUT, 14.0));

                // separator
                ui.add(Separator::default().spacing(24.0));

                // ===== section: license
                ui.label(component::text_en("License", 18.0));
                ui.add_space(6.0);
                ui.label(component::text_en(TAPE_LICENSE, 14.0));

                // if ui.link(prefab::text_en("Click here for details", 14.0)).clicked() {
                //     ui.ctx().output_mut(|o| {
                //         o.open_url = Some(OpenUrl::same_tab(TAPE_LICENSE_URL));
                //     });
                // }
            });
        },
    );
}