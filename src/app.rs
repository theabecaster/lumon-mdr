use crate::theme::Palette;
use rand::{Rng, rng};
use crossterm::event::{MouseEvent, MouseEventKind, KeyCode};
use std::collections::HashMap;

pub enum AppState {
    Login,    
    Loading, 
    Main,
    Prize,  
}

// Structure to track data for each container
pub struct DataContainer {
    pub count: u16,            
    pub progress: f32,         
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
    pub username: String,           
    pub username_cursor: usize,     
    pub show_login_error: bool,     
    pub loading_timer: u16,
    pub progress_percentage: f32,
    pub completion_delay: u8,
    pub completion_timer: u8,          
    pub prize_name: String,            
    pub animation_counter: u32,
    pub mouse_position: Option<(u16, u16)>,
    pub last_clicked: Option<(u16, u16)>,
    pub containers: Vec<DataContainer>,
    pub replaced_numbers: HashMap<(usize, usize), u16>,  
    pub window_size_warning: bool,
    pub show_size_warning: bool,
    pub current_width: u16,
    pub current_height: u16,
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
            state: AppState::Login,   
            username: String::new(),
            username_cursor: 0,
            show_login_error: false,  
            loading_timer: 0,
            progress_percentage: 0.0,
            completion_delay: 0,
            completion_timer: 0,
            prize_name: String::new(),
            animation_counter: 0,
            mouse_position: None,
            last_clicked: None,
            containers,
            replaced_numbers: HashMap::new(),
            window_size_warning: false,
            show_size_warning: false,
            current_width: 0,
            current_height: 0,
         }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        // If size warning is showing, dismiss it and process no further
        if self.show_size_warning {
            self.show_size_warning = false;
            return;
        }

        match self.state {
            AppState::Login => {
                // Any input clears previous error
                self.show_login_error = false;
                
                match key {
                    KeyCode::Char(c) => {
                        if self.username.len() < 25 { // Limit username length
                            self.username.insert(self.username_cursor, c);
                            self.username_cursor += 1;
                        }
                    },
                    KeyCode::Backspace => {
                        if self.username_cursor > 0 {
                            self.username_cursor -= 1;
                            self.username.remove(self.username_cursor);
                        }
                    },
                    KeyCode::Delete => {
                        if self.username_cursor < self.username.len() {
                            self.username.remove(self.username_cursor);
                        }
                    },
                    KeyCode::Left => {
                        if self.username_cursor > 0 {
                            self.username_cursor -= 1;
                        }
                    },
                    KeyCode::Right => {
                        if self.username_cursor < self.username.len() {
                            self.username_cursor += 1;
                        }
                    },
                    KeyCode::Enter => {
                        if !self.username.trim().is_empty() {
                            self.state = AppState::Loading;
                        } else {
                            // Set error flag if username is empty
                            self.show_login_error = true;
                        }
                    },
                    KeyCode::Esc => {
                        self.running = false;
                    },
                    _ => {}
                }
            },
            AppState::Prize => {
                match key {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.running = false;
                    },
                    KeyCode::Char('r') | KeyCode::Enter | KeyCode::Char(' ') => {
                        // Reset all containers and go back to main screen
                        self.reset_containers();
                        self.state = AppState::Main;
                    },
                    _ => {}
                }
            },
            _ => {
                // Existing key handling
                match key {
                    KeyCode::Char('q') => {
                        self.running = false;
                    },
                    // R key resets all containers
                    KeyCode::Char('r') => self.reset_containers(),
                    _ => {}
                }
            }
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
        
        match self.state {
            AppState::Loading => {
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
            },
            AppState::Main => {
                // Check if all containers are filled
                if self.is_all_complete() {
                    // Start completion timer
                    self.completion_timer += 1;
                    
                    // After 3 seconds (9 ticks at 300ms per tick), transition to prize screen
                    if self.completion_timer >= 9 {
                        self.select_random_prize();
                        self.state = AppState::Prize;
                    }
                } else {
                    // Reset timer if containers are not full
                    self.completion_timer = 0;
                }
            },
            _ => {}
        }
    }

    // Reset all containers to zero
    pub fn reset_containers(&mut self) {
        for container in &mut self.containers {
            container.count = 0;
            container.progress = 0.0;
        }
    }

    // Check if all containers are 100% full
    pub fn is_all_complete(&self) -> bool {
        self.containers.iter().all(|container| container.is_full())
    }

    // Select a random prize for the user
    pub fn select_random_prize(&mut self) {
        let prizes = [
            "Waffle Party",
            "Melon Bar",
            "Finger Trap",
            "Caricature Portrait",
            "Dance Experience",
            "Music/Dance Experience",
            "Wellness Session",
            "Coffee Cozy",
            "Choice of Desk Toy",
        ];
        
        let mut rng = rng();
        let prize_idx = rng.random_range(0..prizes.len());
        self.prize_name = prizes[prize_idx].to_string();
    }
}