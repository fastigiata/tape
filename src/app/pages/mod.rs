mod home;

use eframe::egui::{Rect, Ui};
use crate::app::core::TapeApp;

/// A renderer to render a page
pub type PageRenderer = fn(app: &mut TapeApp, rect: Rect, ui: &mut Ui, frame: &mut eframe::Frame);

pub use home::home_renderer;
