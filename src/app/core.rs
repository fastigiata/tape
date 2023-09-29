use eframe::{CreationContext, egui, glow};
use eframe::egui::{Align, Align2, CentralPanel, FontFamily, FontId, Id, ImageButton, Rect, Sense, Vec2, Visuals};
use crate::app::misc::{prepare_font, IconName, TapeIcon};
use crate::app::pages::{about_renderer, home_renderer, PageRenderer};

// region Constants
const APP_BORDER_RADIUS: f32 = 8.0;
const APP_BANNER_H: f32 = 32.0;
// endregion

// region Helpers
#[derive(Debug, PartialEq)]
pub enum AppState { Idle, Record, Act }

#[derive(Debug, PartialEq)]
pub enum AppRoute { Home, Record, Act, History, About }

impl AppRoute {
    pub fn name(&self) -> String {
        match self {
            AppRoute::Home => "Tape",
            AppRoute::Record => "Record",
            AppRoute::Act => "Act",
            AppRoute::History => "History",
            AppRoute::About => "About",
        }.to_string()
    }

    pub fn window_size(&self) -> Vec2 {
        match self {
            AppRoute::Home => egui::vec2(400.0, 240.0),
            AppRoute::Record => egui::vec2(800.0, 600.0),
            AppRoute::Act => egui::vec2(800.0, 600.0),
            AppRoute::History => egui::vec2(800.0, 600.0),
            AppRoute::About => egui::vec2(400.0, 420.0),
        }
    }
}
// endregion

pub struct TapeApp {
    icons: TapeIcon,
    app_state: AppState,
    app_route: AppRoute,
}

impl TapeApp {
    pub fn new(cc: &CreationContext) -> TapeApp {
        prepare_font(&cc.egui_ctx);

        // not nessary for now
        egui_extras::install_image_loaders(&cc.egui_ctx);

        TapeApp {
            icons: TapeIcon::new(),
            app_state: AppState::Idle,
            app_route: AppRoute::Home,
        }
    }

    /// Set the route of the app
    pub fn set_app_route(&mut self, route: AppRoute) {
        self.app_route = route;
    }

    /// Render the banner
    fn render_banner(&mut self, rect: Rect, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let painter = ui.painter();

        // paint the title
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.app_route.name(),
            FontId::new(20.0, FontFamily::Name("MaShanZheng".into())),
            ui.style().visuals.text_color(),
        );

        // paint the separator line
        painter.line_segment(
            [
                rect.left_bottom() + egui::vec2(1.0, 0.0),
                rect.right_bottom() + egui::vec2(-1.0, 0.0),
            ],
            ui.visuals().widgets.noninteractive.fg_stroke,
        );

        // interact with the banner -- drag to move the window
        let react_area = ui.interact(rect, Id::new("banner"), Sense::click());
        if react_area.is_pointer_button_down_on() {
            frame.drag_window();
        }

        // operate the banner -- dark/light mode, minimize, close
        ui.allocate_ui_at_rect(rect, |ui| {
            // render the buttons on the right side
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);

                // close the window
                if ui.add(ImageButton::new(
                    self.icons.sized_image(IconName::Close, egui::vec2(12.0, 12.0))
                ))
                    .on_hover_text("Close the window").clicked() {
                    frame.close();
                }

                // minimize the window
                if ui.add(ImageButton::new(
                    self.icons.sized_image(IconName::Min, egui::vec2(12.0, 12.0))
                ))
                    .on_hover_text("Minimize the window").clicked() {
                    frame.set_minimized(true);
                }

                // switch between dark & light mode
                if ui.style().visuals.dark_mode {
                    if ui.add(ImageButton::new(
                        self.icons.sized_image(IconName::Light, egui::vec2(12.0, 12.0))
                    ))
                        .on_hover_text("Switch to light mode").clicked() {
                        ui.ctx().set_visuals(Visuals::light());
                    }
                } else {
                    if ui.add(ImageButton::new(
                        self.icons.sized_image(IconName::Dark, egui::vec2(12.0, 12.0))
                    ))
                        .on_hover_text("Switch to dark mode").clicked() {
                        ui.ctx().set_visuals(Visuals::dark());
                    }
                }

                // render the 'back' button if the 'app_route' is not 'Home'
                if self.app_route != AppRoute::Home {
                    // close the window
                    if ui.add(ImageButton::new(
                        self.icons.sized_image(IconName::Back, egui::vec2(12.0, 12.0))
                    ))
                        .on_hover_text("Back to home").clicked() {
                        self.set_app_route(AppRoute::Home);
                    }
                }
            });
        });
    }

    /// Render the outlet
    fn render_outlet(&mut self, rect: Rect, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // get the renderer following the 'app_route'
        let renderer: PageRenderer = match self.app_route {
            AppRoute::Home => home_renderer,
            AppRoute::About => about_renderer,
            // TODO: renderer for other routes
            _ => home_renderer,
        };

        // render the outlet following the 'app_route'
        renderer(self, rect, ui, frame);
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

                // render the banner
                self.render_banner(
                    {
                        let mut rect = app_rect;
                        rect.max.y = rect.min.y + APP_BANNER_H;
                        rect
                    },
                    ui, frame,
                );

                // render the outlet following the 'app_route'
                self.render_outlet(
                    {
                        let mut rect = app_rect;
                        rect.min.y += APP_BANNER_H;
                        rect
                    }, ui, frame,
                );
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