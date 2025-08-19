// src/todo.rs
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub completed: bool,
    pub tags: Vec<String>,
    pub due_date: Option<String>,
    pub severity: Severity,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Todo {
    pub fn new(title: String, created_at: String) -> Self {
        Todo {
            title,
            content: String::new(),
            created_at,
            completed: false,
            tags: Vec::new(),
            due_date: None,
            severity: Severity::Medium,
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

    pub fn set_due_date(&mut self, date_str: &str) -> Result<(), chrono::ParseError> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
        self.due_date = Some(date_str.to_string());
        Ok(())
    }

    pub fn set_severity(&mut self, severity: Severity) {
        self.severity = severity;
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due_date_str) = &self.due_date {
            if let Ok(date) = NaiveDate::parse_from_str(due_date_str, "%Y-%m-%d") {
                return date < Local::now().date_naive() && !self.completed;
            }
        }
        false
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "Critical"),
            Severity::High => write!(f, "High"),
            Severity::Medium => write!(f, "Medium"),
            Severity::Low => write!(f, "Low"),
            Severity::Info => write!(f, "Info"),
        }
    }
}