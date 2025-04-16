use crate::{app::App, ui};
use crossterm::event::{self, Event};
use std::time::{Duration, Instant};
use ratatui::Terminal;

pub fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> anyhow::Result<()> {
    // Enable mouse capture when the app starts
    crossterm::execute!(
        std::io::stdout(),
        crossterm::event::EnableMouseCapture
    )?;
    
    // For consistent timing - extremely slow rate for barely perceptible animation
    let tick_rate = Duration::from_millis(300);  // Increased from 100ms
    let mut last_tick = Instant::now();
    
    while app.running {
        // Calculate time until next tick
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_millis(0));
            
        // Draw UI
        terminal.draw(|frame| ui::draw(frame, app))?;
        
        // Poll for events with timeout
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => app.on_key(key.code),
                Event::Mouse(mouse) => app.on_mouse(mouse),
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