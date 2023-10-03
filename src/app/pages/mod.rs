//! This file contains the pages of the app. Each page is a module that contains a renderer function.
mod home;
mod about;
mod record;

use eframe::egui::{Rect, Ui};
use crate::app::core::TapeApp;

/// A renderer to render a page
pub type PageRenderer = fn(app: &mut TapeApp, rect: Rect, ui: &mut Ui, frame: &mut eframe::Frame);

pub use home::home_renderer;
pub use record::record_renderer;
pub use about::about_renderer;
