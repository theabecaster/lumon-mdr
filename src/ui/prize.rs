use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::app::App;

/// Draws the prize screen that appears when all containers reach 100%
pub fn draw_prize_screen<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Check if we have a small window
    let is_small_window = area.height < 15;
    
    // Create layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(if is_small_window { 1 } else { 2 })
        .constraints([
            Constraint::Length(3),   // Congratulations title
            Constraint::Length(1),   // Divider
            Constraint::Length(if is_small_window { 0 } else { 5 }),   // Trophy/celebration graphic (skip in small windows)
            Constraint::Length(if is_small_window { 1 } else { 2 }),   // Space
            Constraint::Length(3),   // Prize announcement
            Constraint::Length(1),   // Space
            Constraint::Length(2),   // Instructions
            Constraint::Min(0),      // Remaining space
        ])
        .split(area);

    // Draw congratulations title
    let title = Paragraph::new("CONGRATULATIONS")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    frame.render_widget(title, layout[0]);

    // Draw divider
    draw_divider(frame, layout[1], app);

    // Draw trophy/celebration graphic only if we have space
    if !is_small_window {
        draw_trophy(frame, layout[2]);
    }

    // Draw prize announcement
    let prize_text = vec![
        Spans::from(Span::styled(
            format!("Employee {} has been awarded:", app.username),
            app.palette.fg_style()
        )),
        Spans::from(""),
        Spans::from(Span::styled(
            format!(">> {} <<", app.prize_name),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        )),
    ];
    
    let prize_para = Paragraph::new(prize_text)
        .alignment(Alignment::Center);
    frame.render_widget(prize_para, layout[4]);
    
    // Draw instructions (simplified for small windows)
    let instructions = if is_small_window {
        vec![
            Spans::from(Span::styled(
                "Press [R]/[ENTER] to reset, [Q]/[ESC] to exit",
                app.palette.fg_style()
            ))
        ]
    } else {
        vec![
            Spans::from(Span::styled(
                "Press [R] or [ENTER] to reset and return to work",
                app.palette.fg_style()
            )),
            Spans::from(Span::styled(
                "Press [Q] or [ESC] to exit",
                app.palette.fg_style()
            )),
        ]
    };
    
    let instructions_para = Paragraph::new(instructions)
        .alignment(Alignment::Center);
    frame.render_widget(instructions_para, layout[6]);
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

/// Draw trophy/celebration graphic
fn draw_trophy<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let trophy = vec![
        "    ___________    ",
        "   '._==_==_=_.'   ",
        "   .-\\:      /-.   ",
        "  | (|:.     |) |  ",
        "   '-|:.     |-'   ",
        "     \\::.    /     ",
        "      '::. .'      ",
        "        ) (        ",
        "      _.' '._      ",
        "     '-------'     ",
    ];
    
    // Convert to spans with centered alignment and yellow color
    let trophy_spans: Vec<Spans> = trophy
        .iter()
        .map(|&line| {
            Spans::from(Span::styled(
                line,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            ))
        })
        .collect();
    
    // Determine how many lines we can show
    let max_lines = area.height.min(trophy.len() as u16);
    let trophy_spans = trophy_spans.into_iter().take(max_lines as usize).collect::<Vec<_>>();
    
    // Center the trophy
    let trophy_para = Paragraph::new(trophy_spans)
        .alignment(Alignment::Center);
    
    frame.render_widget(trophy_para, area);
} 