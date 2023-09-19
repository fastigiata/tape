use eframe::{CreationContext, egui, glow};
use eframe::egui::{Align, Align2, CentralPanel, FontId, Id, ImageButton, Sense, Vec2, Visuals};
use crate::app::icons::{IconName, TapeIcon};

// region Constants
const APP_BORDER_RADIUS: f32 = 8.0;
const APP_BANNER_H: f32 = 32.0;
// endregion

// region Helpers
#[derive(Debug, PartialEq)]
pub enum AppState { Idle, Record, Act }

#[derive(Debug, PartialEq)]
pub enum AppRoute { Home, Record, Act, List, About }

impl AppRoute {
    pub fn name(&self) -> String {
        match self {
            AppRoute::Home => "Home",
            AppRoute::Record => "Record",
            AppRoute::Act => "Act",
            AppRoute::List => "List",
            AppRoute::About => "About",
        }.to_string()
    }

    pub fn window_size(&self) -> Vec2 {
        match self {
            // AppRoute::Home => egui::vec2(400.0, 240.0),
            AppRoute::Home => egui::vec2(800.0, 600.0),
            AppRoute::Record => egui::vec2(800.0, 600.0),
            AppRoute::Act => egui::vec2(800.0, 600.0),
            AppRoute::List => egui::vec2(800.0, 600.0),
            AppRoute::About => egui::vec2(800.0, 600.0),
        }
    }
}
// endregion

pub struct TapeApp {
    icons: TapeIcon,
    app_state: AppState,
    app_route: AppRoute,
    dark_mode: bool,
}

impl TapeApp {
    pub fn new(_cc: &CreationContext) -> TapeApp {
        TapeApp {
            icons: TapeIcon::new(),
            app_state: AppState::Idle,
            app_route: AppRoute::Home,
            dark_mode: false,
        }
    }

    /// Render the banner
    fn render_banner(&self, rect: eframe::epaint::Rect, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let painter = ui.painter();

        // paint the title
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.app_route.name(),
            FontId::proportional(20.0),
            ui.style().visuals.text_color(),
        );

        // paint the separator line
        painter.line_segment(
            [
                rect.left_bottom() + egui::vec2(1.0, 0.0),
                rect.right_bottom() + egui::vec2(-1.0, 0.0),
            ],
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        // interact with the banner -- drag to move the window
        let react_area = ui.interact(rect, Id::new("banner"), Sense::click());
        if react_area.is_pointer_button_down_on() {
            frame.drag_window();
        }

        // operate the banner -- dark/light mode, minimize, close
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(8.0);
                ui.visuals_mut().button_frame = false;

                // close the window
                if ui.add(ImageButton::new(
                    self.icons.get(IconName::Close).texture_id(ui.ctx()),
                    egui::vec2(16.0, 16.0),
                )).on_hover_text("Close the window").clicked() {
                    frame.close();
                }

                // minimize the window
                if ui.add(ImageButton::new(
                    self.icons.get(IconName::Min).texture_id(ui.ctx()),
                    egui::vec2(16.0, 16.0),
                )).on_hover_text("Minimize the window").clicked() {
                    frame.set_minimized(true);
                }

                // switch between dark & light mode
                if ui.style().visuals.dark_mode {
                    if ui.add(ImageButton::new(
                        self.icons.get(IconName::Light).texture_id(ui.ctx()),
                        egui::vec2(16.0, 16.0),
                    )).on_hover_text("Switch to light mode").clicked() {
                        ui.ctx().set_visuals(Visuals::light());
                    }
                } else {
                    if ui.add(ImageButton::new(
                        self.icons.get(IconName::Dark).texture_id(ui.ctx()),
                        egui::vec2(16.0, 16.0),
                    )).on_hover_text("Switch to dark mode").clicked() {
                        ui.ctx().set_visuals(Visuals::dark());
                    }
                }
            });
        });
    }

    /// Render the outlet
    fn render_outlet(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // TODO: render the outlet following the 'app_route'

        ui.label("This is just the contents of the window.");
    }

    /// Render the entire app (including the banner and outlet)
    fn render_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // set the window size according to the 'app_route'
        frame.set_window_size(self.app_route.window_size());

        CentralPanel::default()
            .frame(egui::Frame {
                fill: ctx.style().visuals.window_fill(),
                rounding: APP_BORDER_RADIUS.into(),
                stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
                outer_margin: 0.5.into(),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();

                let banner_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + APP_BANNER_H;
                    rect
                };

                // render the banner
                self.render_banner(banner_rect, ui, frame);

                // render the outlet following the 'app_route'
                self.render_outlet(ui, frame);
            });
    }
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