// src/note.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub tags: Vec<String>,
}

impl Note {
    pub fn new(title: String, created_at: String) -> Self {
        Note {
            title,
            content: String::new(),
            created_at,
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }
}