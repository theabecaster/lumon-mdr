use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use rand::{Rng, SeedableRng, rngs::StdRng};

/// Renders the main screen with data bins
pub fn draw_main_screen<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(8),
        ])
        .split(area);

    // Title bar
    let title = Block::default()
        .borders(Borders::ALL)
        .title("Lumon MDR")
        .style(app.palette.fg_style());
    frame.render_widget(title, main_layout[0]);

    // Main content area
    let main_content = Block::default()
        .borders(Borders::ALL)
        .style(app.palette.fg_style());
    
    // Clone the rect for inner area calculation before rendering the block
    let content_area = main_layout[1];
    frame.render_widget(main_content.clone(), content_area);
    
    // Generate and render grid of numbers in main content area
    let inner_area = main_content.inner(content_area);
    draw_number_grid(frame, inner_area, app);

    // Bottom containers section
    let bottom_area = main_layout[2];
    
    // Calculate container sizes
    let total_gap_width = 4 * 5;
    let available_width = bottom_area.width.saturating_sub(total_gap_width);
    let container_width = available_width / 5;
    
    // Container layout
    let containers = Layout::default()
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
        .split(bottom_area);
    
    // Save container positions for click detection
    let container_positions = [
        containers[0], containers[2], containers[4], containers[6], containers[8]
    ];
    
    // If there was a click, check if it was on a container
    if let Some((click_x, click_y)) = app.last_clicked {
        // First, check if the click was on a container
        let mut clicked_container = false;
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
                clicked_container = true;
                break;
            }
        }
        
        // If click wasn't on a container, check for magnified numbers
        if !clicked_container {
            // Reset the click after handling - clicking on numbers is handled in draw_number_grid
            let app_ptr = app as *const App as *mut App;
            unsafe {
                (*app_ptr).last_clicked = None;
            }
        }
    }
    
    // Render all containers
    let container_indices = [0, 2, 4, 6, 8];
    for (idx, &container_idx) in container_indices.iter().enumerate() {
        let container = containers[container_idx];
        
        // Container internal layout
        let container_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),                // Number square
                Constraint::Length(3),                // Progress bar (3 lines for top/middle/bottom)
                Constraint::Min(0),                   // Remaining space
            ])
            .split(container);
        
        // Get data for this container
        let container_data = &app.containers[idx];
        let entry_count = container_data.count;
        let progress = container_data.progress;
        
        // Draw number square
        let square = Block::default()
            .borders(Borders::ALL)
            .style(app.palette.fg_style());
            
        let inner_square = square.inner(container_layout[0]);
        
        frame.render_widget(square, container_layout[0]);
        
        // Draw number
        let count_text = Paragraph::new(format!("{}", entry_count))
            .alignment(ratatui::layout::Alignment::Center)
            .style(app.palette.fg_style());
            
        let center_y = inner_square.y + inner_square.height / 2;
        let centered_rect = Rect::new(
            inner_square.x,
            center_y,
            inner_square.width,
            1
        );
        
        frame.render_widget(count_text, centered_rect);
        
        // Progress bar
        let progress_percentage = progress as u16;
        let progress_width = container_layout[1].width.saturating_sub(2);
        let filled = (progress_width as f32 * (progress_percentage as f32 / 100.0)) as u16;
        
        // Create progress bar with percentage in the middle and borders
        let mut top_border = String::new();
        let mut bar = String::new();
        let mut bottom_border = String::new();
        
        // Format the percentage text
        let percentage_text = format!("{}%", progress_percentage);
        let percentage_len = percentage_text.len() as u16;
        
        // Calculate position to center the percentage text
        let text_start = (progress_width - percentage_len) / 2;
        let text_end = text_start + percentage_len;
        
        // Top border
        top_border.push('┌');
        for _ in 0..progress_width {
            top_border.push('─');
        }
        top_border.push('┐');
        
        // Middle with percentage
        bar.push('│');
        for i in 0..progress_width {
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
        for _ in 0..progress_width {
            bottom_border.push('─');
        }
        bottom_border.push('┘');
        
        let progress_text = Paragraph::new(vec![
            Spans::from(top_border),
            Spans::from(bar),
            Spans::from(bottom_border),
        ])
        .alignment(ratatui::layout::Alignment::Center)
        .style(app.palette.fg_style());
        
        frame.render_widget(progress_text, container_layout[1]);
    }
}

/// Draw a grid of random numbers in the main content area
fn draw_number_grid<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Number of numbers to display horizontally and vertically
    // Increase spacing between numbers (6 spaces horizontally, 2 lines vertically)
    let horizontal_spacing = 6;  // Increased from 4
    let vertical_spacing = 2;    // Increased from 1
    
    // Calculate max columns and rows that will fit within the area
    // Subtract 2 from width and height to create a safe margin
    let max_width = area.width.saturating_sub(2);
    let max_height = area.height.saturating_sub(1);
    
    let num_cols = max_width / horizontal_spacing;
    let num_rows = max_height / vertical_spacing;
    
    // Static seed to ensure consistent random numbers between renders
    let mut base_rng = StdRng::seed_from_u64(42);
    
    // Use animation counter for continuous movement - with extremely slow rate
    let time = app.animation_counter as f32 * 0.01;  // Reduced from 0.04
    
    // Mouse interaction parameters
    let mouse_position = app.mouse_position;
    let max_influence_distance = 10.0; // Maximum distance at which mouse affects numbers
    let max_scale_factor = 2.0;       // Maximum size increase (2x = double size)
    
    // Track magnified numbers if there was a click
    let was_click = app.last_clicked.is_some();
    let mut magnified_positions: Vec<(usize, usize, u16)> = Vec::new(); // (col, row, digit)
    
    // Process and render the numbers
    for row in 0..num_rows as usize {
        for col in 0..num_cols as usize {
            // Get digit - either from the replaced positions map or generate with base RNG
            let digit = if let Some(replaced_digit) = app.get_replaced_number(col, row) {
                replaced_digit
            } else {
                base_rng.random_range(0..=9)
            };
            
            // Calculate base position with increased spacing
            let base_x = area.x + (col as u16) * horizontal_spacing + 2;
            let base_y = area.y + (row as u16) * vertical_spacing + (vertical_spacing / 2);
            
            // Create a unique seed for each number based on position and value
            let unique_seed = (row as f32 * 0.73) + (col as f32 * 0.37) + (digit as f32 * 0.19);
            
            // Determine if this number moves horizontally or vertically (never both)
            // Use a unique property of each position to decide
            let moves_horizontally = ((row + col + digit as usize) % 2) == 0;
            
            // Calculate movement with same amplitude (0.8) for both directions
            let movement = (time + unique_seed).sin() * 0.8;
            
            // Apply movement to either horizontal or vertical, but not both
            let x_offset = if moves_horizontally { movement.round() as i16 } else { 0 };
            let y_offset = if !moves_horizontally { movement.round() as i16 } else { 0 };
            
            // Apply the offset while ensuring we stay in bounds
            let x = (base_x as i16 + x_offset).max(area.x as i16).min((area.x + max_width - 1) as i16) as u16;
            let y = (base_y as i16 + y_offset).max(area.y as i16).min((area.y + max_height - 1) as i16) as u16;
            
            // Calculate distance from mouse cursor (if available)
            let scale_factor = if let Some((mouse_x, mouse_y)) = mouse_position {
                // Calculate Euclidean distance
                let dx = (x as f32) - (mouse_x as f32);
                let dy = (y as f32) - (mouse_y as f32);
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Scale factor decreases with distance (1.0 = normal size, max_scale_factor = largest size)
                if distance < max_influence_distance {
                    // Linear scaling: closer = larger
                    let scale = 1.0 + (max_scale_factor - 1.0) * (1.0 - distance / max_influence_distance);
                    
                    // If this was a click and the number is magnified, add it to our collection
                    if was_click && scale > 1.5 && 
                       app.last_clicked.map_or(false, |(cx, cy)| {
                            // Ensure click was in the grid area
                            cx >= area.x && cx < area.x + area.width && 
                            cy >= area.y && cy < area.y + area.height
                       }) {
                        magnified_positions.push((col, row, digit));
                    }
                    
                    scale
                } else {
                    1.0 // Default scale (no change)
                }
            } else {
                1.0 // No mouse position available
            };
            
            // Make sure we're still within bounds
            if x < area.x + area.width && y < area.y + area.height {
                // Render the digit with scaling
                if scale_factor > 1.0 {
                    // For larger scale, use a custom approach: render multiple characters
                    let scaled_size = (scale_factor.round() as usize).max(1);
                    
                    if scaled_size == 2 {
                        // 2x scale - use a 2x2 grid of the digit
                        let grid_positions = [
                            (x, y),                  // Top-left
                            (x.saturating_add(1), y),            // Top-right
                            (x, y.saturating_add(1)),            // Bottom-left
                            (x.saturating_add(1), y.saturating_add(1)),  // Bottom-right
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
                        let digit_rect = Rect::new(x, y, 1, 1);
                        let digit_text = Paragraph::new(format!("{}", digit))
                            .style(app.palette.fg_style());
                        frame.render_widget(digit_text, digit_rect);
                    }
                } else {
                    // No scaling - render as normal
                    let digit_rect = Rect::new(x, y, 1, 1);
                    let digit_text = Paragraph::new(format!("{}", digit))
                        .style(app.palette.fg_style());
                    frame.render_widget(digit_text, digit_rect);
                }
            }
        }
    }
    
    // If we collected magnified numbers and there was a click, process them
    if !magnified_positions.is_empty() && was_click {
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