use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
};
use std::rc::Rc;

use crate::app::{App, DataContainer};
use rand::{Rng, SeedableRng, rngs::StdRng};

/// Renders the main screen with data bins
pub fn draw_main_screen<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Create the main layout
    let main_layout = create_main_layout(area);

    // Draw title bar
    draw_title_bar(frame, main_layout[0], app);
    
    // Draw thick divider under title bar
    draw_horizontal_divider(frame, main_layout[1], app, true);

    // Draw main content (number grid)
    let content_area = main_layout[2];
    let main_content = Block::default()
        .style(app.palette.fg_style());
    
    frame.render_widget(main_content.clone(), content_area);
    let inner_area = main_content.inner(content_area);
    draw_number_grid(frame, inner_area, app);

    // Draw thick horizontal divider above data containers
    draw_horizontal_divider(frame, main_layout[3], app, true);

    // Top padding is empty
    
    // Draw data containers
    draw_data_containers(frame, main_layout[5], app);
    
    // Bottom padding is empty
    
    // Draw thin horizontal divider below data containers
    draw_horizontal_divider(frame, main_layout[7], app, false);
    
    // Draw footer text
    draw_footer_text(frame, main_layout[8], app);
}

/// Creates the main layout structure
fn create_main_layout(area: Rect) -> Rc<[Rect]> {
    // Calculate padding - we want equal spacing above and below containers
    let container_height = 6;  // Actual height needed for containers
    let padding = 1;           // Equal padding above and below
    
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),           // Title bar
            Constraint::Length(1),           // Title divider
            Constraint::Min(10),             // Main content (grid)
            Constraint::Length(1),           // Thick divider
            Constraint::Length(padding),     // Top padding
            Constraint::Length(container_height), // Container section
            Constraint::Length(padding),     // Bottom padding
            Constraint::Length(1),           // Thin divider
            Constraint::Length(1),           // Footer text
        ])
        .split(area)
}

/// Draw the title bar at the top of the screen
fn draw_title_bar<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    let title = Block::default()
        .borders(Borders::ALL)
        .style(app.palette.fg_style());
    frame.render_widget(title, area);
}

/// Draw footer text centered below the skinny divider
fn draw_footer_text<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Generate a memory address based on app pointer address
    let app_ptr = app as *const App;
    let memory_addr1 = format!("0x{:016x}", app_ptr as usize);
    
    // Generate a second memory address based on containers pointer
    let containers_ptr = &app.containers as *const Vec<DataContainer>;
    let memory_addr2 = format!("0x{:016x}", containers_ptr as usize);
    
    let footer_text = format!("{} : {}", memory_addr1, memory_addr2);
    
    let footer_widget = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .style(app.palette.fg_style());
    
    frame.render_widget(footer_widget, area);
}

/// Draw the data containers at the bottom of the screen
fn draw_data_containers<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Calculate container sizes
    let total_gap_width = 4 * 5;
    let available_width = area.width.saturating_sub(total_gap_width);
    let container_width = available_width / 5;
    
    // Create container layout
    let containers = create_container_layout(area, container_width);
    
    // Get container positions for click detection
    let container_positions = [
        containers[0], containers[2], containers[4], containers[6], containers[8]
    ];
    
    // Process clicks on containers
    process_container_clicks(app, &container_positions);
    
    // Render all containers
    let container_indices = [0, 2, 4, 6, 8];
    for (idx, &container_idx) in container_indices.iter().enumerate() {
        let container_rect = containers[container_idx];
        draw_single_container(frame, container_rect, idx, &app.containers[idx], app);
    }
}

/// Create the horizontal layout for containers with gaps
fn create_container_layout(area: Rect, container_width: u16) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(container_width),
            Constraint::Length(5),
            Constraint::Length(container_width),
            Constraint::Length(5),
            Constraint::Length(container_width),
            Constraint::Length(5),
            Constraint::Length(container_width),
            Constraint::Length(5),
            Constraint::Length(container_width),
        ])
        .split(area)
}

/// Process mouse clicks on containers
fn process_container_clicks(app: &App, container_positions: &[Rect]) {
    if let Some((click_x, click_y)) = app.last_clicked {
        for (idx, &container_rect) in container_positions.iter().enumerate() {
            if click_x >= container_rect.x && 
               click_x < container_rect.x + container_rect.width &&
               click_y >= container_rect.y && 
               click_y < container_rect.y + container_rect.height {
                // Click was on this container
                let app_ptr = app as *const App as *mut App;
                unsafe {
                    (*app_ptr).add_to_container(idx, 3);
                }
                break;
            }
        }
    }
}

/// Draw a single data container
fn draw_single_container<B: Backend>(
    frame: &mut Frame<B>, 
    container: Rect, 
    idx: usize, 
    container_data: &DataContainer, 
    app: &App
) {
    // Container internal layout
    let container_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),                // Number square
            Constraint::Length(3),                // Progress bar (3 lines for top/middle/bottom)
            Constraint::Min(0),                   // Remaining space
        ])
        .split(container);
    
    // Draw number square
    draw_container_number(frame, container_layout[0], idx, app);
    
    // Draw progress bar
    draw_progress_bar(frame, container_layout[1], container_data.progress, app);
}

/// Draw the container number square
fn draw_container_number<B: Backend>(frame: &mut Frame<B>, area: Rect, idx: usize, app: &App) {
    let square = Block::default()
        .borders(Borders::ALL)
        .style(app.palette.fg_style());
        
    let inner_square = square.inner(area);
    
    frame.render_widget(square, area);
    
    // Draw number
    let count_text = Paragraph::new(format!("0{}", idx + 1))
        .alignment(Alignment::Center)
        .style(app.palette.fg_style());
        
    let center_y = inner_square.y + inner_square.height / 2;
    let centered_rect = Rect::new(
        inner_square.x,
        center_y,
        inner_square.width,
        1
    );
    
    frame.render_widget(count_text, centered_rect);
}

/// Draw a container progress bar with percentage
fn draw_progress_bar<B: Backend>(
    frame: &mut Frame<B>, 
    area: Rect, 
    progress: f32, 
    app: &App
) {
    let progress_percentage = progress as u16;
    let progress_width = area.width.saturating_sub(2);
    let filled = (progress_width as f32 * (progress_percentage as f32 / 100.0)) as u16;
    
    // Format the percentage text
    let percentage_text = format!("{}%", progress_percentage);
    let percentage_len = percentage_text.len() as u16;
    
    // Calculate position to center the percentage text
    let text_start = (progress_width - percentage_len) / 2;
    let text_end = text_start + percentage_len;
    
    // Create progress bar parts
    let (top_border, bar, bottom_border) = create_progress_bar_parts(
        progress_width, 
        filled, 
        &percentage_text, 
        text_start, 
        text_end
    );
    
    let progress_text = Paragraph::new(vec![
        Spans::from(top_border),
        Spans::from(bar),
        Spans::from(bottom_border),
    ])
    .alignment(Alignment::Center)
    .style(app.palette.fg_style());
    
    frame.render_widget(progress_text, area);
}

/// Create the three components of a progress bar: top border, middle with fill, and bottom border
fn create_progress_bar_parts(
    width: u16, 
    filled: u16, 
    percentage_text: &str, 
    text_start: u16, 
    text_end: u16
) -> (String, String, String) {
    let mut top_border = String::new();
    let mut bar = String::new();
    let mut bottom_border = String::new();
    
    // Top border
    top_border.push('┌');
    for _ in 0..width {
        top_border.push('─');
    }
    top_border.push('┐');
    
    // Middle with percentage
    bar.push('│');
    for i in 0..width {
        // Check if we're in the range where the percentage text should be displayed
        if i >= text_start && i < text_end {
            let char_idx = (i - text_start) as usize;
            if char_idx < percentage_text.len() {
                bar.push(percentage_text.chars().nth(char_idx).unwrap());
            }
        } else if i < filled {
            bar.push('█');
        } else {
            bar.push(' ');
        }
    }
    bar.push('│');
    
    // Bottom border
    bottom_border.push('└');
    for _ in 0..width {
        bottom_border.push('─');
    }
    bottom_border.push('┘');
    
    (top_border, bar, bottom_border)
}

/// Draw a grid of random numbers in the main content area
fn draw_number_grid<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Calculate grid dimensions
    let (num_cols, num_rows, horizontal_spacing, vertical_spacing) = 
        calculate_grid_dimensions(area);

    // Create RNG with static seed for consistent numbers between renders
    let mut base_rng = StdRng::seed_from_u64(42);
    
    // Animation time based on app counter
    let time = app.animation_counter as f32 * 0.01;
    
    // Track magnified numbers if there was a click
    let was_click = app.last_clicked.is_some();
    let mut magnified_positions: Vec<(usize, usize, u16)> = Vec::new();
    
    // Process and render each number in the grid
    for row in 0..num_rows as usize {
        for col in 0..num_cols as usize {
            let digit = get_digit(app, col, row, &mut base_rng);
            
            let (x, y) = calculate_number_position(
                col, row, area, horizontal_spacing, vertical_spacing, time, digit
            );
            
            let scale_factor = calculate_scale_factor(app, x, y);
            
            // Track magnified numbers on click
            if was_click && scale_factor > 1.5 && is_click_in_grid_area(app, area) {
                magnified_positions.push((col, row, digit));
            }
            
            // Render the digit
            render_digit(frame, x, y, digit, scale_factor, area, app);
        }
    }
    
    // Process clicked numbers
    process_clicked_numbers(app, magnified_positions);
}

/// Calculate the grid dimensions based on available area
fn calculate_grid_dimensions(area: Rect) -> (u16, u16, u16, u16) {
    let horizontal_spacing = 6;  // Space between numbers horizontally
    let vertical_spacing = 2;    // Space between numbers vertically
    
    // Calculate max columns and rows that will fit
    let max_width = area.width.saturating_sub(2);
    let max_height = area.height.saturating_sub(1);
    
    let num_cols = max_width / horizontal_spacing;
    let num_rows = max_height / vertical_spacing;
    
    (num_cols, num_rows, horizontal_spacing, vertical_spacing)
}

/// Get the digit to display at a specific position
fn get_digit(app: &App, col: usize, row: usize, rng: &mut StdRng) -> u16 {
    if let Some(replaced_digit) = app.get_replaced_number(col, row) {
        replaced_digit
    } else {
        rng.random_range(0..=9)
    }
}

/// Calculate the position of a number in the grid, including animation
fn calculate_number_position(
    col: usize, 
    row: usize, 
    area: Rect, 
    horizontal_spacing: u16, 
    vertical_spacing: u16,
    time: f32,
    digit: u16
) -> (u16, u16) {
    // Calculate base position
    let base_x = area.x + (col as u16) * horizontal_spacing + 2;
    let base_y = area.y + (row as u16) * vertical_spacing + (vertical_spacing / 2);
    
    // Create a unique seed for animation
    let unique_seed = (row as f32 * 0.73) + (col as f32 * 0.37) + (digit as f32 * 0.19);
    
    // Determine movement direction
    let moves_horizontally = ((row + col + digit as usize) % 2) == 0;
    
    // Calculate animation movement
    let movement = (time + unique_seed).sin() * 0.8;
    
    // Apply movement to either horizontal or vertical, but not both
    let x_offset = if moves_horizontally { movement.round() as i16 } else { 0 };
    let y_offset = if !moves_horizontally { movement.round() as i16 } else { 0 };
    
    // Apply the offset while ensuring we stay in bounds
    let max_width = area.width.saturating_sub(2);
    let max_height = area.height.saturating_sub(1);
    
    let x = (base_x as i16 + x_offset).max(area.x as i16).min((area.x + max_width - 1) as i16) as u16;
    let y = (base_y as i16 + y_offset).max(area.y as i16).min((area.y + max_height - 1) as i16) as u16;
    
    (x, y)
}

/// Calculate scale factor based on mouse proximity
fn calculate_scale_factor(app: &App, x: u16, y: u16) -> f32 {
    if let Some((mouse_x, mouse_y)) = app.mouse_position {
        // Maximum distance at which mouse affects numbers
        let max_influence_distance = 10.0;
        // Maximum size increase (2x = double size)
        let max_scale_factor = 2.0;
        
        // Calculate Euclidean distance
        let dx = (x as f32) - (mouse_x as f32);
        let dy = (y as f32) - (mouse_y as f32);
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Scale factor decreases with distance (1.0 = normal size, max_scale_factor = largest size)
        if distance < max_influence_distance {
            // Linear scaling: closer = larger
            1.0 + (max_scale_factor - 1.0) * (1.0 - distance / max_influence_distance)
        } else {
            1.0 // Default scale (no change)
        }
    } else {
        1.0 // No mouse position available
    }
}

/// Check if click was in the grid area
fn is_click_in_grid_area(app: &App, area: Rect) -> bool {
    app.last_clicked.map_or(false, |(cx, cy)| {
        cx >= area.x && cx < area.x + area.width && 
        cy >= area.y && cy < area.y + area.height
    })
}

/// Render a digit with optional scaling
fn render_digit<B: Backend>(
    frame: &mut Frame<B>, 
    x: u16, 
    y: u16, 
    digit: u16, 
    scale_factor: f32, 
    area: Rect,
    app: &App
) {
    // Make sure we're still within bounds
    if x < area.x + area.width && y < area.y + area.height {
        if scale_factor > 1.0 {
            // For larger scale, use a custom approach
            let scaled_size = (scale_factor.round() as usize).max(1);
            
            if scaled_size == 2 {
                // 2x scale - use a 2x2 grid of the digit
                let grid_positions = [
                    (x, y),                                // Top-left
                    (x.saturating_add(1), y),              // Top-right
                    (x, y.saturating_add(1)),              // Bottom-left
                    (x.saturating_add(1), y.saturating_add(1)),    // Bottom-right
                ];
                
                for &pos in &grid_positions {
                    if pos.0 < area.x + area.width && pos.1 < area.y + area.height {
                        let digit_rect = Rect::new(pos.0, pos.1, 1, 1);
                        let digit_text = Paragraph::new(format!("{}", digit))
                            .style(app.palette.fg_style());
                        frame.render_widget(digit_text, digit_rect);
                    }
                }
            } else {
                // Default: just render at normal size
                render_single_digit(frame, x, y, digit, app);
            }
        } else {
            // No scaling - render as normal
            render_single_digit(frame, x, y, digit, app);
        }
    }
}

/// Render a single digit at the specified position
fn render_single_digit<B: Backend>(frame: &mut Frame<B>, x: u16, y: u16, digit: u16, app: &App) {
    let digit_rect = Rect::new(x, y, 1, 1);
    let digit_text = Paragraph::new(format!("{}", digit))
        .style(app.palette.fg_style());
    frame.render_widget(digit_text, digit_rect);
}

/// Process clicked numbers and update the app state
fn process_clicked_numbers(app: &App, magnified_positions: Vec<(usize, usize, u16)>) {
    if !magnified_positions.is_empty() && app.last_clicked.is_some() {
        // Sum all collected magnified numbers
        let sum: u16 = magnified_positions.iter().map(|&(_, _, digit)| digit).sum();
        
        // Extract just the positions for replacing
        let positions_to_replace: Vec<(usize, usize)> = 
            magnified_positions.iter().map(|&(col, row, _)| (col, row)).collect();
        
        // Add to a random non-full container and replace numbers
        let app_ptr = app as *const App as *mut App;
        unsafe {
            // Add the sum to a container
            (*app_ptr).add_to_random_non_full_container(sum);
            
            // Replace each collected number with a new random one
            (*app_ptr).replace_numbers(positions_to_replace);
        }
    }
}

/// Draw a horizontal divider line that spans the full width of the screen
fn draw_horizontal_divider<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App, thick: bool) {
    // Create a horizontal line using appropriate box drawing characters
    let mut divider = String::new();
    
    // Fill the entire width of the screen with appropriate line characters
    let line_char = if thick { '━' } else { '─' }; // Heavy or light horizontal line
    
    for _ in 0..area.width {
        divider.push(line_char);
    }
    
    // Create a paragraph with the divider
    let divider_widget = Paragraph::new(divider)
        .style(app.palette.fg_style());
    
    frame.render_widget(divider_widget, area);
} 