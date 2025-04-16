// UI module for rendering the application
use ratatui::{
    Frame,
    backend::Backend,
    widgets::Block,
};

use crate::app::{App, AppState};

mod loading;
mod main_screen;

pub use loading::LOADING_MESSAGES;

/// Main drawing function for the UI
pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let area = frame.size();

    // Set background
    frame.render_widget(Block::default().style(app.palette.bg_style()), area);

    // Draw appropriate screen based on app state
    match app.state {
        AppState::Loading => loading::draw_loading_screen(frame, area, app),
        AppState::Main => main_screen::draw_main_screen(frame, area, app),
    }
} 