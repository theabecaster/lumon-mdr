use lumon_mdr::{app::App, input, theme};
use ratatui::backend::CrosstermBackend;
use crossterm::{
    execute, 
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetSize}, 
    event::DisableMouseCapture
};
use std::io;

fn main() -> anyhow::Result<()> {
    // Set desired window size (columns, rows)
    let desired_width = 120;
    let desired_height = 40;
    
    // Try to set terminal size (this works in many but not all terminals)
    let _ = execute!(io::stdout(), SetSize(desired_width, desired_height));
    
    // terminal bootstrap
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // run the TUI
    let mut app = App::new(theme::detect());
    let result = input::event_loop(&mut terminal, &mut app);
    
    // restore tty
    crossterm::terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(), 
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    
    // Return any error that might have occurred
    result
}