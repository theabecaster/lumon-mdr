use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

/// Draws the login screen with username input
pub fn draw_login_screen<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Check if we have a small window
    let is_small_window = area.height < 20;
    
    // Create layout with adaptive constraints
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(if is_small_window { 1 } else { 2 })
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Length(1),   // Divider
            Constraint::Length(if is_small_window { 0 } else { 6 }),   // Logo (hide in small window)
            Constraint::Length(if is_small_window { 1 } else { 2 }),   // Space
            Constraint::Length(3),   // Login instructions
            Constraint::Length(3),   // Input field
            Constraint::Length(1),   // Error message space
            Constraint::Length(if is_small_window { 0 } else { 2 }),   // Space (reduce in small window)
            Constraint::Length(if is_small_window { 3 } else { 6 }),   // Usage instructions (reduced in small window)
            Constraint::Min(0),      // Remaining space
        ])
        .split(area);

    // Draw title
    let title = Paragraph::new("LUMON INDUSTRIES TERMINAL")
        .alignment(Alignment::Center)
        .style(app.palette.fg_style());
    frame.render_widget(title, layout[0]);

    // Draw divider
    draw_divider(frame, layout[1], app);

    // Draw simple logo only if not in small window mode
    if !is_small_window {
        draw_simplified_logo(frame, layout[2], app);
    }

    // Draw login instructions
    let login_text = vec![
        Spans::from(Span::styled(
            "Enter your employee identification name:",
            app.palette.fg_style()
        )),
        Spans::from(Span::styled(
            "Press ENTER to continue.",
            app.palette.fg_style()
        )),
    ];
    
    let login_instructions = Paragraph::new(login_text)
        .alignment(Alignment::Center);
    frame.render_widget(login_instructions, layout[4]);

    // Draw input field with cursor
    let input_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(layout[5])[1];

    // Create input text with cursor
    let input_text = if app.username_cursor < app.username.len() {
        // Split the username at cursor position
        let (before, after) = app.username.split_at(app.username_cursor);
        let after_chars: Vec<char> = after.chars().collect();
        
        // Make sure there are characters to extract
        if !after_chars.is_empty() {
            // Extract the character at cursor position
            let cursor_char = after_chars[0];
            
            // Create the after part without the cursor character
            let remaining = &after[cursor_char.len_utf8()..];
            
            // Create spans with the character at cursor position highlighted
            vec![
                Span::styled(before, app.palette.fg_style()),
                Span::styled(
                    cursor_char.to_string(),
                    Style::default().fg(Color::Black).bg(Color::White)
                ),
                Span::styled(remaining, app.palette.fg_style()),
            ]
        } else {
            // Handle case where cursor is at the end
            vec![
                Span::styled(before, app.palette.fg_style()),
                Span::styled(" ", Style::default().bg(Color::White))
            ]
        }
    } else {
        // Cursor is at the end, use a block cursor
        vec![
            Span::styled(app.username.as_str(), app.palette.fg_style()),
            Span::styled(" ", Style::default().bg(Color::White))
        ]
    };
    
    let input = Paragraph::new(Spans::from(input_text))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(app.palette.fg_style()))
        .style(app.palette.fg_style().add_modifier(Modifier::BOLD));
    
    frame.render_widget(input, input_area);
    
    // Draw error message if needed
    if app.show_login_error {
        let error_text = "ERROR: Employee name cannot be empty";
        let error_message = Paragraph::new(error_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        frame.render_widget(error_message, layout[6]);
    }
    
    // Draw app usage instructions (simplified for small windows)
    let usage_text = if is_small_window {
        vec![
            Spans::from(Span::styled(
                "CONTROLS: [q] Quit [r] Reset",
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)
            )),
            Spans::from(Span::styled(
                "Use mouse to select numbers and data bins",
                app.palette.fg_style()
            )),
        ]
    } else {
        vec![
            Spans::from(Span::styled(
                "APPLICATION CONTROLS",
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)
            )),
            Spans::from(""),
            Spans::from(Span::styled(
                "During operation: [q] Quit [r] Reset containers",
                app.palette.fg_style()
            )),
            Spans::from(Span::styled(
                "Mouse: Click on numbers to select them, click on bins to add data",
                app.palette.fg_style()
            )),
            Spans::from(Span::styled(
                "Complete tasks by collecting numbers into the data refinement bins",
                app.palette.fg_style()
            )),
        ]
    };
    
    let usage_instructions = Paragraph::new(usage_text)
        .alignment(Alignment::Center);
    frame.render_widget(usage_instructions, layout[8]);
}

/// Draw a divider line
fn draw_divider<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    let mut divider = String::new();
    for _ in 0..area.width {
        divider.push('━');
    }
    
    let divider_widget = Paragraph::new(divider).style(app.palette.fg_style());
    frame.render_widget(divider_widget, area);
}

/// Draw simplified Lumon logo 
fn draw_simplified_logo<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    let logo = vec![
        " _       _    _ __  __  ___  _   _ ",
        " | |     | |  | |  \\/  |/ _ \\| \\ | |",
        "| |     | |  | | \\  / | | | |  \\| |",
        " | |     | |  | | |\\/| | | | | . ` |",
        "| |___  | |__| | |  | | |_| | |\\  |",
        " |_____|  \\____/|_|  |_|\\___/|_| \\_|",
    ];
    
    // Convert to spans
    let logo_spans: Vec<Spans> = logo
        .iter()
        .map(|&line| {
            Spans::from(Span::styled(
                line,
                app.palette.fg_style().add_modifier(Modifier::BOLD)
            ))
        })
        .collect();
    
    // Center the logo
    let logo_para = Paragraph::new(logo_spans)
        .alignment(Alignment::Center);
    
    frame.render_widget(logo_para, area);
} 