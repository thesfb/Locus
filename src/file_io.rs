// src/file_io.rs
use crate::{note::Note, todo::Todo};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct AppData {
    pub notes: Vec<Note>,
    pub todos: Vec<Todo>,
}

pub struct FileIO {
    data_dir: PathBuf,
    app_file: PathBuf,
}

impl FileIO {
    pub fn new() -> Result<Self, io::Error> {
        let mut data_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        data_dir.push(".terminal_notes");
        fs::create_dir_all(&data_dir)?;

        let app_file = data_dir.join("data.json");
        Ok(FileIO { data_dir, app_file })
    }

    pub fn save_data(&self, notes: &[Note], todos: &[Todo]) -> Result<(), io::Error> {
        let app_data = AppData {
            notes: notes.to_vec(),
            todos: todos.to_vec(),
        };
        let json = serde_json::to_string_pretty(&app_data)?;
        fs::write(&self.app_file, json)?;
        Ok(())
    }

    pub fn load_data(&self) -> Result<(Vec<Note>, Vec<Todo>), io::Error> {
        if !self.app_file.exists() {
            return Ok((Vec::new(), Vec::new()));
        }

        let contents = fs::read_to_string(&self.app_file)?;
        let app_data: AppData = serde_json::from_str(&contents)?;
        Ok((app_data.notes, app_data.todos))
    }

    pub fn backup_data(&self) -> Result<PathBuf, io::Error> {
        if !self.app_file.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No data to backup",
            ));
        }

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_file = self
            .data_dir
            .join(format!("backup_{}.json", timestamp));
        fs::copy(&self.app_file, &backup_file)?;
        Ok(backup_file)
    }

    pub fn export_data(
        &self,
        format: &str,
        path: &Path,
        notes: &[Note],
        todos: &[Todo],
    ) -> Result<(), io::Error> {
        match format {
            "json" => self.export_json(path),
            "csv" => self.export_csv(path, notes, todos),
            "markdown" | "md" => self.export_markdown(path, notes, todos),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported format",
            )),
        }
    }

    fn export_json(&self, path: &Path) -> Result<(), io::Error> {
        fs::copy(&self.app_file, path)?;
        Ok(())
    }

    fn export_csv(
        &self,
        path: &Path,
        notes: &[Note],
        todos: &[Todo],
    ) -> Result<(), io::Error> {
        let mut writer = csv::Writer::from_path(path)?;
        writer.write_record(&["Type", "Title", "Content", "Created At", "Completed"])?;

        for note in notes {
            writer.write_record(&[
                "Note",
                &note.title,
                &note.content,
                &note.created_at,
                "",
            ])?;
        }

        for todo in todos {
            writer.write_record(&[
                "Todo",
                &todo.title,
                &todo.content,
                &todo.created_at,
                &todo.completed.to_string(),
            ])?;
        }
        writer.flush()?;
        Ok(())
    }

    fn export_markdown(
        &self,
        path: &Path,
        notes: &[Note],
        todos: &[Todo],
    ) -> Result<(), io::Error> {
        let mut file = File::create(path)?;

        writeln!(file, "# Notes\n")?;
        for note in notes {
            writeln!(file, "## {}", note.title)?;
            writeln!(file, "*Created: {}*\n", note.created_at)?;
            writeln!(file, "{}\n---\n", note.content)?;
        }

        writeln!(file, "# Todos\n")?;
        for todo in todos {
            let status = if todo.completed { "✓" } else { "☐" };
            writeln!(file, "## {} {}", status, todo.title)?;
            writeln!(file, "*Created: {}*\n", todo.created_at)?;
            writeln!(file, "{}\n---\n", todo.content)?;
        }
        Ok(())
    }
}