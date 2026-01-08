use crate::todo::TodoItem;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

#[derive(Debug)]
pub enum AppMode {
    Normal,
    Insert,
    Help,
}

#[derive(Debug)]
pub struct App {
    pub todos: Vec<TodoItem>,
    pub list_state: ListState,
    pub mode: AppMode,
    pub input: Input,
    next_id: usize,
    pub should_quit: bool,
    data_file: String,
}

impl App {
    pub fn new() -> Result<Self> {
        let data_file = Self::get_data_file_path()?;
        let todos = Self::load_todos(&data_file)?;
        let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        let mut app = Self {
            todos,
            list_state: ListState::default(),
            mode: AppMode::Normal,
            input: Input::default(),
            next_id,
            should_quit: false,
            data_file,
        };

        if !app.todos.is_empty() {
            app.list_state.select(Some(0));
        }

        Ok(app)
    }

    fn get_data_file_path() -> Result<String> {
        // Try XDG_DATA_HOME first, fall back to ~/.local/share
        let data_dir = if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
            PathBuf::from(xdg_data_home).join("oxitodo")
        } else {
            let home_dir = std::env::var("HOME")
                .map_err(|_| color_eyre::eyre::eyre!("Could not find HOME directory"))?;

            PathBuf::from(home_dir)
                .join(".local")
                .join("share")
                .join("oxitodo")
        };

        // Create the directory if it doesn't exist
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }

        let data_file = data_dir.join("todos.json");
        Ok(data_file.to_string_lossy().to_string())
    }

    fn load_todos(file_path: &str) -> Result<Vec<TodoItem>> {
        if Path::new(file_path).exists() {
            let content = fs::read_to_string(file_path)?;
            let todos: Vec<TodoItem> = serde_json::from_str(&content)?;
            Ok(todos)
        } else {
            Ok(vec![])
        }
    }

    fn save_todos(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.todos)?;
        fs::write(&self.data_file, json)?;
        Ok(())
    }

    pub fn add_todo(&mut self, text: String) {
        if !text.trim().is_empty() {
            let todo = TodoItem::new(self.next_id, text.trim().to_string());
            self.todos.push(todo);
            self.next_id += 1;

            // Select the new item
            self.list_state.select(Some(self.todos.len() - 1));

            let _ = self.save_todos();
        }
    }

    pub fn toggle_current_todo(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if let Some(todo) = self.todos.get_mut(selected) {
                todo.toggle_completion();
                let _ = self.save_todos();
            }
        }
    }

    pub fn delete_current_todo(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.todos.len() {
                self.todos.remove(selected);

                // Adjust selection
                if self.todos.is_empty() {
                    self.list_state.select(None);
                } else if selected >= self.todos.len() {
                    self.list_state.select(Some(self.todos.len() - 1));
                }

                let _ = self.save_todos();
            }
        }
    }

    pub fn next_item(&mut self) {
        if self.todos.is_empty() {
            return;
        }

        let selected = match self.list_state.selected() {
            Some(i) => {
                if i >= self.todos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(selected));
    }

    pub fn previous_item(&mut self) {
        if self.todos.is_empty() {
            return;
        }

        let selected = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.todos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(selected));
    }

    pub fn handle_key_event(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.mode {
            AppMode::Normal => match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('i') => self.mode = AppMode::Insert,
                KeyCode::Char('?') => self.mode = AppMode::Help,
                KeyCode::Char(' ') => self.toggle_current_todo(),
                KeyCode::Char('d') => self.delete_current_todo(),
                KeyCode::Up | KeyCode::Char('k') => self.previous_item(),
                KeyCode::Down | KeyCode::Char('j') => self.next_item(),
                _ => {}
            },
            AppMode::Insert => match key.code {
                KeyCode::Esc => {
                    self.mode = AppMode::Normal;
                    self.input.reset();
                }
                KeyCode::Enter => {
                    let input_text = self.input.value().to_string();
                    self.add_todo(input_text);
                    self.input.reset();
                    self.mode = AppMode::Normal;
                }
                _ => {
                    self.input.handle_event(&Event::Key(key));
                }
            },
            AppMode::Help => match key.code {
                KeyCode::Esc | KeyCode::Char('?') => self.mode = AppMode::Normal,
                _ => {}
            },
        }
    }

    pub fn completed_count(&self) -> usize {
        self.todos.iter().filter(|t| t.is_completed()).count()
    }

    pub fn total_count(&self) -> usize {
        self.todos.len()
    }
}
