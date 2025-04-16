use crate::theme::Palette;
use rand::{Rng, rng};
use crossterm::event::{MouseEvent, MouseEventKind};
use std::collections::HashMap;

pub enum AppState {
    Loading, 
    Main,
}

// Structure to track data for each container
pub struct DataContainer {
    pub count: u16,            // Current count (0-100)
    pub progress: f32,         // Progress percentage (0.0-100.0)
}

impl DataContainer {
    pub fn new() -> Self {
        Self {
            count: 0,
            progress: 0.0,
        }
    }
    
    // Add value to container, respecting max of 100
    pub fn add(&mut self, value: u16) {
        self.count = (self.count + value).min(100);
        self.progress = self.count as f32;
    }
    
    // Check if container is full
    pub fn is_full(&self) -> bool {
        self.count >= 100
    }
}

pub struct App {
    pub palette: Palette,
    pub running: bool,
    pub state: AppState,
    pub loading_timer: u16,
    pub progress_percentage: f32,
    pub completion_delay: u8,
    pub animation_counter: u32,
    pub mouse_position: Option<(u16, u16)>,
    pub last_clicked: Option<(u16, u16)>,
    pub containers: Vec<DataContainer>,
    pub replaced_numbers: HashMap<(usize, usize), u16>,  // Map of (col, row) to new digit
}

impl App {
    pub fn new(palette: Palette) -> Self {
        // Initialize 5 data containers all at 0
        let mut containers = Vec::with_capacity(5);
        for _ in 0..5 {
            containers.push(DataContainer::new());
        }
        
        Self { 
            palette, 
            running: true, 
            state: AppState::Loading,
            loading_timer: 0,
            progress_percentage: 0.0,
            completion_delay: 0,
            animation_counter: 0,
            mouse_position: None,
            last_clicked: None,
            containers,
            replaced_numbers: HashMap::new(),
         }
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Char('q') => {
                self.running = false;
            },
            // Number keys 1-5 add to specific containers
            crossterm::event::KeyCode::Char('1') => self.add_to_container(0, 5),
            crossterm::event::KeyCode::Char('2') => self.add_to_container(1, 5),
            crossterm::event::KeyCode::Char('3') => self.add_to_container(2, 5),
            crossterm::event::KeyCode::Char('4') => self.add_to_container(3, 5),
            crossterm::event::KeyCode::Char('5') => self.add_to_container(4, 5),
            // Space adds random values
            crossterm::event::KeyCode::Char(' ') => self.add_random(),
            // R key resets all containers
            crossterm::event::KeyCode::Char('r') => self.reset_containers(),
            _ => {}
        }
    }
    
    pub fn on_mouse(&mut self, event: MouseEvent) {
        // Update current mouse position without affecting animation
        self.mouse_position = Some((event.column, event.row));
        
        // Handle mouse clicks
        match event.kind {
            MouseEventKind::Down(_) => {
                self.last_clicked = Some((event.column, event.row));
                // Actual click processing is done in the UI rendering
            }
            _ => {}
        }
    }
    
    // Replace a number at a specific position with a new random value
    pub fn replace_number(&mut self, col: usize, row: usize) {
        let mut rng = rng();
        let new_digit = rng.random_range(0..=9);
        self.replaced_numbers.insert((col, row), new_digit);
    }
    
    // Replace multiple numbers at once
    pub fn replace_numbers(&mut self, positions: Vec<(usize, usize)>) {
        for (col, row) in positions {
            self.replace_number(col, row);
        }
    }
    
    // Get a replaced number if it exists
    pub fn get_replaced_number(&self, col: usize, row: usize) -> Option<u16> {
        self.replaced_numbers.get(&(col, row)).copied()
    }
    
    // Add a value to a specific container
    pub fn add_to_container(&mut self, container_idx: usize, value: u16) {
        if container_idx < self.containers.len() {
            self.containers[container_idx].add(value);
            // Reset the last click to avoid repeated processing
            self.last_clicked = None;
        }
    }
    
    // Add a random value to a random container
    pub fn add_random(&mut self) {
        let mut rng = rng();
        let container_idx = rng.random_range(0..self.containers.len());
        let value = rng.random_range(1..=10);
        
        self.add_to_container(container_idx, value);
    }
    
    // Add a value to a random non-full container
    pub fn add_to_random_non_full_container(&mut self, value: u16) {
        // Find non-full containers
        let non_full_indices: Vec<usize> = self.containers.iter()
            .enumerate()
            .filter(|(_, container)| !container.is_full())
            .map(|(idx, _)| idx)
            .collect();
            
        // If there are non-full containers, add to a random one
        if !non_full_indices.is_empty() {
            let mut rng = rng();
            let idx = rng.random_range(0..non_full_indices.len());
            let container_idx = non_full_indices[idx];
            
            self.add_to_container(container_idx, value);
        }
        
        // Reset click regardless
        self.last_clicked = None;
    }

    pub fn tick(&mut self) {
        // Increment animation counter at a steady rate
        self.animation_counter = self.animation_counter.wrapping_add(1);
        
        if let AppState::Loading = self.state {
            self.loading_timer += 1;

            if self.loading_timer >= 3 {
                self.loading_timer = 0;
                
                if self.progress_percentage >= 100.0 {
                    self.progress_percentage = 100.0;
                    self.completion_delay += 1;
                    
                    if self.completion_delay >= 2 {
                        self.state = AppState::Main;
                    }
                } else {
                    let mut rng = rng();
                    let progress_increment = rng.random_range(0.0..13.0);
                    
                    let new_progress = self.progress_percentage + progress_increment;
                    if new_progress > 100.0 {
                        self.progress_percentage = 100.0;
                    } else {
                        self.progress_percentage = new_progress;
                    }
                }
            }
        }
    }

    // Reset all containers to zero
    pub fn reset_containers(&mut self) {
        for container in &mut self.containers {
            container.count = 0;
            container.progress = 0.0;
        }
    }
}