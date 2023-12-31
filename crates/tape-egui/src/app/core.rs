//! This file contains the core of the app.
use eframe::{CreationContext, egui, glow};
use eframe::egui::{Align, Align2, CentralPanel, Color32, FontFamily, FontId, Id, ImageButton, Rect, Sense, Visuals};
use tape_core::act::Actor;
use tape_core::record::Recorder;
use crate::app::prelude::{prepare_font, IconName, TapeIcon};
use crate::app::pages::{about_renderer, home_renderer, PageRenderer, record_renderer};
use crate::app::component::{text_en};

// region Constants
const APP_BORDER_RADIUS: f32 = 8.0;
const APP_BANNER_H: f32 = 32.0;
// endregion

// region Helpers
#[derive(Debug, PartialEq)]
pub enum AppState { Idle, Record, Act }

impl AppState {
    pub fn display_name(&self) -> String {
        match self {
            AppState::Idle => "idle",
            AppState::Record => "recording",
            AppState::Act => "acting",
        }.to_string()
    }

    pub fn color(&self) -> Color32 {
        match self {
            AppState::Idle => Color32::from_rgb(94, 185, 110),
            AppState::Record => Color32::from_rgb(252, 83, 86),
            AppState::Act => Color32::from_rgb(243, 143, 49),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AppRoute { Home, Record, Act, History, Setting, About }

impl AppRoute {
    pub fn name(&self) -> String {
        match self {
            AppRoute::Home => "Tape",
            AppRoute::Record => "Record",
            AppRoute::Act => "Act",
            AppRoute::History => "History",
            AppRoute::Setting => "Setting",
            AppRoute::About => "About",
        }.to_string()
    }
}
// endregion

pub struct TapeApp {
    icons: TapeIcon,
    app_state: AppState,
    app_route: AppRoute,
    worker_record: Recorder,
    worker_act: Actor,
}

impl TapeApp {
    pub fn new(cc: &CreationContext) -> TapeApp {
        prepare_font(&cc.egui_ctx);

        // for image support
        egui_extras::install_image_loaders(&cc.egui_ctx);

        TapeApp {
            icons: TapeIcon::new(),
            app_state: AppState::Idle,
            app_route: AppRoute::Home,
            worker_record: Default::default(),
            worker_act: Default::default(),
        }
    }

    /// Set the state of the app
    pub fn set_app_state(&mut self, state: AppState) {
        self.app_state = state;
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

        // left side buttons (app state)
        ui.allocate_ui_at_rect(
            {
                let mut l_rect = rect;
                l_rect.max.x = (rect.min.x + rect.max.x) / 2.0;
                l_rect
            },
            |ui| {
                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    ui.add_space(8.0);

                    // show the logo
                    ui.add(self.icons.sized_image(IconName::TAPE, egui::vec2(24.0, 24.0)));

                    // show the app state circle
                    // ui.painter().circle_filled(
                    //     rect.left_center() + egui::vec2(48.0, 0.0),
                    //     8.0,
                    //     self.app_state.color(),
                    // );

                    // show the app state text
                    // ui.add_space(8.0);
                    ui.label(text_en(self.app_state.display_name(), 16.0));
                });
            },
        );

        // right side buttons (close, minimize, dark/light mode)
        ui.allocate_ui_at_rect(
            {
                let mut r_rect = rect;
                r_rect.min.x = (rect.min.x + rect.max.x) / 2.0;
                r_rect
            },
            |ui| {
                // render the buttons on the right side
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    ui.visuals_mut().button_frame = false;
                    ui.add_space(8.0);

                    // close the window
                    if ui.add(ImageButton::new(
                        self.icons.sized_image(IconName::Close, egui::vec2(14.0, 14.0))
                    )).on_hover_text("Close the window").clicked() {
                        frame.close();
                    }

                    // minimize the window
                    if ui.add(ImageButton::new(
                        self.icons.sized_image(IconName::Min, egui::vec2(14.0, 14.0))
                    )).on_hover_text("Minimize the window").clicked() {
                        frame.set_minimized(true);
                    }

                    // switch between dark & light mode
                    if ui.style().visuals.dark_mode {
                        if ui.add(ImageButton::new(
                            self.icons.sized_image(IconName::Light, egui::vec2(14.0, 14.0))
                        ))
                            .on_hover_text("Switch to light mode").clicked() {
                            ui.ctx().set_visuals(Visuals::light());
                        }
                    } else {
                        if ui.add(ImageButton::new(
                            self.icons.sized_image(IconName::Dark, egui::vec2(14.0, 14.0))
                        ))
                            .on_hover_text("Switch to dark mode").clicked() {
                            ui.ctx().set_visuals(Visuals::dark());
                        }
                    }

                    // render the 'back' button if the 'app_route' is not 'Home'
                    if self.app_route != AppRoute::Home {
                        // close the window
                        if ui.add(ImageButton::new(
                            self.icons.sized_image(IconName::Back, egui::vec2(14.0, 14.0))
                        ))
                            .on_hover_text("Back to home").clicked() {
                            self.set_app_route(AppRoute::Home);
                        }
                    }
                });
            },
        );
    }

    /// Render the outlet
    fn render_outlet(&mut self, rect: Rect, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // get the renderer following the 'app_route'
        // TODO: renderer for other routes
        let renderer: PageRenderer = match self.app_route {
            AppRoute::Home => home_renderer,
            AppRoute::Record => record_renderer,
            AppRoute::About => about_renderer,
            _ => home_renderer,
        };

        // render the outlet following the 'app_route'
        renderer(self, rect, ui, frame);
    }

    /// Render the entire app (including the banner and outlet)
    fn render_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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