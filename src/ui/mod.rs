// UI module for rendering the application
use ratatui::{
    Frame,
    backend::Backend,
    widgets::Block,
};

use crate::app::{App, AppState};

mod loading;
mod main_screen;
mod login;
mod prize;

pub use loading::LOADING_MESSAGES;

/// Main drawing function for the UI
pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let area = frame.size();

    // Set background
    frame.render_widget(Block::default().style(app.palette.bg_style()), area);

    // Check if terminal is too small for any UI
    let absolute_min_width = 20;
    let absolute_min_height = 10;
    
    if area.width < absolute_min_width || area.height < absolute_min_height {
        // Draw a minimal message for extremely small terminals
        let min_message = "Terminal\ntoo small";
        let min_widget = ratatui::widgets::Paragraph::new(min_message)
            .alignment(ratatui::layout::Alignment::Center)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
                  .add_modifier(ratatui::style::Modifier::BOLD));
        
        frame.render_widget(min_widget, area);
        return;
    }
    
    // Check if we should show the size warning message
    if app.show_size_warning {
        // Draw a warning message about optimal window size
        use crate::input::{DESIRED_WIDTH, DESIRED_HEIGHT};
        let warning = format!(
            "⚠️ Window Size Warning ⚠️\n\nOptimal size: {}x{}\nCurrent size: {}x{}\n\nPress any key to continue",
            DESIRED_WIDTH, DESIRED_HEIGHT, app.current_width, app.current_height
        );
        
        // Create a floating box in the center of the screen
        let warning_width = 50.min(area.width - 4);
        let warning_height = 10.min(area.height - 4);
        let warning_x = (area.width - warning_width) / 2;
        let warning_y = (area.height - warning_height) / 2;
        
        let warning_area = ratatui::layout::Rect::new(
            warning_x, warning_y, warning_width, warning_height
        );
        
        let warning_box = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black));
            
        let warning_widget = ratatui::widgets::Paragraph::new(warning)
            .alignment(ratatui::layout::Alignment::Center)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
            .block(warning_box);
            
        frame.render_widget(warning_widget, warning_area);
        return;
    }

    // Draw appropriate screen based on app state
    match app.state {
        AppState::Login => login::draw_login_screen(frame, area, app),
        AppState::Loading => loading::draw_loading_screen(frame, area, app),
        AppState::Main => main_screen::draw_main_screen(frame, area, app),
        AppState::Prize => prize::draw_prize_screen(frame, area, app),
    }
} 