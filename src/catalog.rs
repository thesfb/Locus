// src/catalog.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CatalogItem {
    pub name: String,
    pub description: String,
    pub url: String,
    pub tags: Vec<String>,
}

impl CatalogItem {
    pub fn new(name: String, description: String, url: String) -> Self {
        CatalogItem {
            name,
            description,
            url,
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