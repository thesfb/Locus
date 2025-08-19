// src/app.rs
use crate::file_io::FileIO;
use crate::note::Note;
use crate::todo::Todo;
use chrono::Local;
use std::io;

pub enum AppMode {
    MainMenu,
    Normal,
    Command,
    Editing,
    Help,
    Renaming,
}

pub enum AppSection {
    Notes,
    Todos,
    Help,
}

pub struct App {
    pub section: AppSection,
    pub notes: Vec<Note>,
    pub todos: Vec<Todo>,
    pub selected_note: Option<usize>,
    pub selected_todo: Option<usize>,
    pub current_note: Option<usize>,
    pub current_todo: Option<usize>,
    pub selected_menu_item: usize,
    pub mode: AppMode,
    pub command_buffer: String,
    pub status_message: Option<String>,
    pub file_io: FileIO,
    pub unsaved_changes: bool,
}

impl App {
    pub fn new() -> Result<Self, io::Error> {
        let file_io = FileIO::new()?;
        let (notes, todos) = file_io.load_data()?;
        Ok(App {
            section: AppSection::Notes,
            notes,
            todos,
            selected_note: None,
            selected_todo: None,
            current_note: None,
            current_todo: None,
            selected_menu_item: 0,
            mode: AppMode::MainMenu,
            command_buffer: String::new(),
            status_message: None,
            file_io,
            unsaved_changes: false,
        })
    }

    pub fn on_tick(&mut self) {}

    pub fn next_menu_item(&mut self) {
        self.selected_menu_item = (self.selected_menu_item + 1) % 3;
    }

    pub fn previous_menu_item(&mut self) {
        self.selected_menu_item = if self.selected_menu_item == 0 {
            2
        } else {
            self.selected_menu_item - 1
        };
    }

    pub fn next_note(&mut self) {
        let next = match self.selected_note {
            Some(i) if i >= self.notes.len() - 1 => Some(0),
            Some(i) => Some(i + 1),
            None if !self.notes.is_empty() => Some(0),
            _ => None,
        };
        self.selected_note = next;
    }

    pub fn previous_note(&mut self) {
        let prev = match self.selected_note {
            Some(0) => Some(self.notes.len() - 1),
            Some(i) => Some(i - 1),
            None if !self.notes.is_empty() => Some(self.notes.len() - 1),
            _ => None,
        };
        self.selected_note = prev;
    }

    pub fn next_todo(&mut self) {
        let next = match self.selected_todo {
            Some(i) if i >= self.todos.len() - 1 => Some(0),
            Some(i) => Some(i + 1),
            None if !self.todos.is_empty() => Some(0),
            _ => None,
        };
        self.selected_todo = next;
    }

    pub fn previous_todo(&mut self) {
        let prev = match self.selected_todo {
            Some(0) => Some(self.todos.len() - 1),
            Some(i) => Some(i - 1),
            None if !self.todos.is_empty() => Some(self.todos.len() - 1),
            _ => None,
        };
        self.selected_todo = prev;
    }

    pub fn select_menu_item(&mut self) {
        match self.selected_menu_item {
            0 => {
                self.section = AppSection::Notes;
                self.mode = AppMode::Normal;
                self.status_message = Some("Notes section".to_string());
            }
            1 => {
                self.section = AppSection::Todos;
                self.mode = AppMode::Normal;
                self.status_message = Some("Todo section".to_string());
            }
            2 => {
                self.section = AppSection::Help;
                self.mode = AppMode::Help;
                self.status_message = Some("Help section".to_string());
            }
            _ => {}
        }
    }

    pub fn execute_command(&mut self) {
        let cmd = self.command_buffer.trim().to_string();
        self.command_buffer.clear();

        let (count, command) = Self::parse_command_count(&cmd);

        match command.as_str() {
            "nn" => (0..count).for_each(|_| self.create_new_note()),
            "ntodo" => (0..count).for_each(|_| self.create_new_todo()),
            "del" => (0..count).for_each(|_| self.delete_current_item()),
            "rnm" => self.start_rename(),
            "mm" => self.go_to_main_menu(),
            "?" => self.show_help(),
            "save" | "w" => self.save_data_with_status(),
            "backup" => self.backup_data_with_status(),
            "export-md" | "export-markdown" => self.export_data_with_status("markdown"),
            "export-csv" => self.export_data_with_status("csv"),
            "q" | "quit" => self.handle_quit(),
            "q!" => {}
            _ => self.status_message = Some(format!("Unknown command: {}", command)),
        }

        if command != "?" {
            self.mode = AppMode::Normal;
        }
    }

    fn parse_command_count(cmd: &str) -> (usize, String) {
        let mut count = 1;
        let mut command = cmd.to_string();

        if let Some(index) = cmd.find(|c: char| !c.is_numeric()) {
            if index > 0 {
                if let Ok(parsed_count) = cmd[..index].parse::<usize>() {
                    count = parsed_count;
                    command = cmd[index..].to_string();
                }
            }
        }
        (count, command)
    }

    pub fn create_new_note(&mut self) {
        let now = Local::now();
        let new_note =
            Note::new(format!("Note {}", self.notes.len() + 1), now.to_rfc3339());
        self.notes.push(new_note);
        self.selected_note = Some(self.notes.len() - 1);
        self.current_note = self.selected_note;
        self.section = AppSection::Notes;
        self.mode = AppMode::Editing;
        self.status_message = Some("New note created".to_string());
        self.unsaved_changes = true;
    }

    pub fn create_new_todo(&mut self) {
        let now = Local::now();
        let new_todo =
            Todo::new(format!("Todo {}", self.todos.len() + 1), now.to_rfc3339());
        self.todos.push(new_todo);
        self.selected_todo = Some(self.todos.len() - 1);
        self.current_todo = self.selected_todo;
        self.section = AppSection::Todos;
        self.mode = AppMode::Editing;
        self.status_message = Some("New todo created".to_string());
        self.unsaved_changes = true;
    }

    pub fn insert_char(&mut self, c: char) {
        match self.section {
            AppSection::Notes => {
                if let Some(note) = self.current_note.and_then(|i| self.notes.get_mut(i)) {
                    note.content.push(c);
                    self.unsaved_changes = true;
                }
            }
            AppSection::Todos => {
                if let Some(todo) = self.current_todo.and_then(|i| self.todos.get_mut(i)) {
                    todo.content.push(c);
                    self.unsaved_changes = true;
                }
            }
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.section {
            AppSection::Notes => {
                if let Some(note) = self.current_note.and_then(|i| self.notes.get_mut(i)) {
                    note.content.pop();
                    self.unsaved_changes = true;
                }
            }
            AppSection::Todos => {
                if let Some(todo) = self.current_todo.and_then(|i| self.todos.get_mut(i)) {
                    todo.content.pop();
                    self.unsaved_changes = true;
                }
            }
            _ => {}
        }
    }

    pub fn insert_new_line(&mut self) {
        match self.section {
            AppSection::Notes => {
                if let Some(note) = self.current_note.and_then(|i| self.notes.get_mut(i)) {
                    note.content.push('\n');
                    self.unsaved_changes = true;
                }
            }
            AppSection::Todos => {
                if let Some(todo) = self.current_todo.and_then(|i| self.todos.get_mut(i)) {
                    todo.content.push('\n');
                    self.unsaved_changes = true;
                }
            }
            _ => {}
        }
    }

    pub fn toggle_todo_completion(&mut self) {
        if let Some(todo) = self.selected_todo.and_then(|i| self.todos.get_mut(i)) {
            todo.completed = !todo.completed;
            self.unsaved_changes = true;
            self.status_message = Some(if todo.completed {
                "Todo marked as completed".to_string()
            } else {
                "Todo marked as incomplete".to_string()
            });
        }
    }

    pub fn save_data(&self) -> Result<(), io::Error> {
        self.file_io.save_data(&self.notes, &self.todos)
    }

    pub fn backup_data(&self) -> Result<std::path::PathBuf, io::Error> {
        self.file_io.backup_data()
    }

    pub fn export_data(
        &self,
        format: &str,
        path: &std::path::Path,
    ) -> Result<(), io::Error> {
        self.file_io
            .export_data(format, path, &self.notes, &self.todos)
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.unsaved_changes
    }

    pub fn delete_current_item(&mut self) {
        match self.section {
            AppSection::Notes => self.delete_note(),
            AppSection::Todos => self.delete_todo(),
            _ => {}
        }
    }

    fn delete_note(&mut self) {
        if let Some(idx) = self.selected_note {
            if idx < self.notes.len() {
                self.notes.remove(idx);
                self.unsaved_changes = true;
                if self.notes.is_empty() {
                    self.selected_note = None;
                    self.current_note = None;
                } else if idx >= self.notes.len() {
                    self.selected_note = Some(self.notes.len() - 1);
                }
                self.status_message = Some("Note deleted".to_string());
            }
        }
    }

    fn delete_todo(&mut self) {
        if let Some(idx) = self.selected_todo {
            if idx < self.todos.len() {
                self.todos.remove(idx);
                self.unsaved_changes = true;
                if self.todos.is_empty() {
                    self.selected_todo = None;
                    self.current_todo = None;
                } else if idx >= self.todos.len() {
                    self.selected_todo = Some(self.todos.len() - 1);
                }
                self.status_message = Some("Todo deleted".to_string());
            }
        }
    }

    pub fn start_rename(&mut self) {
        self.command_buffer.clear();
        let title = match self.section {
            AppSection::Notes => self
                .selected_note
                .and_then(|i| self.notes.get(i))
                .map(|n| n.title.clone()),
            AppSection::Todos => self
                .selected_todo
                .and_then(|i| self.todos.get(i))
                .map(|t| t.title.clone()),
            _ => None,
        };

        if let Some(title) = title {
            self.command_buffer = title;
            self.mode = AppMode::Renaming;
            self.status_message = Some("Enter new name:".to_string());
        }
    }

    pub fn finish_rename(&mut self, new_name: String) {
        match self.section {
            AppSection::Notes => {
                if let Some(note) = self.selected_note.and_then(|i| self.notes.get_mut(i))
                {
                    note.title = new_name;
                    self.unsaved_changes = true;
                    self.status_message = Some("Note renamed".to_string());
                }
            }
            AppSection::Todos => {
                if let Some(todo) = self.selected_todo.and_then(|i| self.todos.get_mut(i))
                {
                    todo.title = new_name;
                    self.unsaved_changes = true;
                    self.status_message = Some("Todo renamed".to_string());
                }
            }
            _ => {}
        }
        self.mode = AppMode::Normal;
    }

    fn go_to_main_menu(&mut self) {
        self.mode = AppMode::MainMenu;
        self.section = AppSection::Notes;
        self.status_message = Some("Main Menu".to_string());
        self.current_note = None;
        self.current_todo = None;
    }

    fn show_help(&mut self) {
        self.mode = AppMode::Help;
        self.status_message = Some("Help".to_string());
    }

    fn save_data_with_status(&mut self) {
        match self.save_data() {
            Ok(_) => {
                self.status_message = Some("Data saved successfully".to_string());
                self.unsaved_changes = false;
            }
            Err(err) => self.status_message = Some(format!("Error saving data: {}", err)),
        }
    }

    fn backup_data_with_status(&mut self) {
        match self.backup_data() {
            Ok(path) => {
                self.status_message = Some(format!("Backup created at: {:?}", path));
            }
            Err(err) => {
                self.status_message = Some(format!("Error creating backup: {}", err));
            }
        }
    }

    fn export_data_with_status(&mut self, format: &str) {
        let extension = if format == "markdown" { "md" } else { "csv" };
        let path = dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(format!("terminal_notes_export.{}", extension));

        match self.export_data(format, &path) {
            Ok(_) => self.status_message = Some(format!("Exported to: {:?}", path)),
            Err(err) => {
                self.status_message =
                    Some(format!("Error exporting to {}: {}", format, err))
            }
        }
    }

    fn handle_quit(&mut self) {
        if self.unsaved_changes {
            self.status_message =
                Some("Unsaved changes! Use :save first or :q! to force quit".to_string());
        }
    }
}