use eframe::{egui, glow};
use eframe::egui::{Align, Align2, CentralPanel, Color32, CursorIcon, FontId, Id, RichText, Sense, Stroke, Visuals};

// region Constants
const APP_COLOR: Color32 = Color32::from_rgb(252, 252, 252);
const APP_BORDER_WIDTH: f32 = 0.5;
const APP_BORDER_COLOR: Color32 = Color32::from_rgb(0, 0, 0);
const APP_BORDER_RADIUS: f32 = 8.0;
const APP_BANNER_H: f32 = 32.0;
const APP_BANNER_BUTTON_H: f32 = 12.0;
// endregion

// region Helpers
#[derive(Debug, PartialEq)]
pub enum AppState { Idle, Record, Act }

#[derive(Debug, PartialEq)]
pub enum AppRoute { Home, Record, Act, List }

impl AppRoute {
    pub fn name(&self) -> String {
        match self {
            AppRoute::Home => "Home",
            AppRoute::Record => "Record",
            AppRoute::Act => "Act",
            AppRoute::List => "List",
        }.to_string()
    }
}
// endregion

pub struct TapeApp {
    app_state: AppState,
    app_route: AppRoute,
}

impl TapeApp {
    pub fn new() -> TapeApp {
        TapeApp {
            app_state: AppState::Idle,
            app_route: AppRoute::Home,
        }
    }

    /// Render the entire app (including the banner and outlet)
    fn render_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let app_frame = egui::Frame {
            fill: APP_COLOR,
            rounding: APP_BORDER_RADIUS.into(),
            stroke: (APP_BORDER_WIDTH, APP_BORDER_COLOR).into(),
            outer_margin: APP_BORDER_WIDTH.into(),
            ..Default::default()
        };

        CentralPanel::default()
            .frame(app_frame)
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();

                let banner_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + APP_BANNER_H;
                    rect
                };

                self.render_banner(banner_rect, ui, frame);


                ui.label("Hello world!");
            });
    }

    /// Render the banner
    fn render_banner(&self, rect: eframe::epaint::Rect, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let painter = ui.painter();

        // paint the title
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.app_route.name(),
            FontId::proportional(16.0),
            Color32::BLACK,
        );

        // paint the separator line
        painter.line_segment(
            [
                rect.left_bottom() + egui::vec2(1.0, 0.0),
                rect.right_bottom() + egui::vec2(-1.0, 0.0),
            ],
            Stroke::from((APP_BORDER_WIDTH, APP_BORDER_COLOR)),
        );

        // interact with the banner -- drag to move the window
        let react_area = ui.interact(rect, Id::new("banner"), Sense::click());
        if react_area.is_pointer_button_down_on() {
            frame.drag_window();
        }

        // operate the banner -- minimize & close
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);

                // close the window
                if ui.button(RichText::new("❌").size(APP_BANNER_BUTTON_H))
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .on_hover_text_at_pointer("Close the window")
                    .clicked() {
                    frame.close();
                }

                // minimize the window
                if ui.button(RichText::new("🗕").size(APP_BANNER_BUTTON_H))
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .on_hover_text_at_pointer("Minimize the window")
                    .clicked() {
                    frame.set_minimized(true);
                }
            });
        });


        // TODO: https://github.com/emilk/egui/blob/master/examples/custom_window_frame/src/main.rs L123
    }

    /// Render the outlet
    fn render_outlet(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
}

impl eframe::App for TapeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.render_app(ctx, frame);
    }

    fn on_exit(&mut self, _gl: Option<&glow::Context>) {
        println!("App will exit");
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}