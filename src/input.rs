use crate::{app::App, ui};
use crossterm::event::{self, Event};
use crossterm::terminal;
use std::time::{Duration, Instant};
use ratatui::Terminal;

// Define the desired window size
pub const DESIRED_WIDTH: u16 = 120;
pub const DESIRED_HEIGHT: u16 = 40;

pub fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> anyhow::Result<()> {
    // Enable mouse capture when the app starts
    crossterm::execute!(
        std::io::stdout(),
        crossterm::event::EnableMouseCapture
    )?;
    
    // Set size warning flag
    let mut has_shown_size_warning = false;
    
    // For consistent timing - extremely slow rate for barely perceptible animation
    let tick_rate = Duration::from_millis(300);  // Increased from 100ms
    let mut last_tick = Instant::now();
    
    // Check window size and update app status
    check_window_size(app);
    
    while app.running {
        // Calculate time until next tick
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_millis(0));
            
        // Draw UI
        terminal.draw(|frame| ui::draw(frame, app))?;
        
        // Show size warning if needed (only once)
        if app.window_size_warning && !has_shown_size_warning {
            app.show_size_warning = true;
            has_shown_size_warning = true;
        }
        
        // Poll for events with timeout
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => app.on_key(key.code),
                Event::Mouse(mouse) => app.on_mouse(mouse),
                Event::Resize(_, _) => check_window_size(app),
                _ => {}
            }
        }
        
        // Update app state at a fixed tick rate
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
    
    // Disable mouse capture when the app exits
    crossterm::execute!(
        std::io::stdout(),
        crossterm::event::DisableMouseCapture
    )?;
    
    Ok(())
}

// Check if window size matches desired size
fn check_window_size(app: &mut App) {
    if let Ok((width, height)) = terminal::size() {
        app.window_size_warning = width < DESIRED_WIDTH || height < DESIRED_HEIGHT;
        app.current_width = width;
        app.current_height = height;
    }
}