#![forbid(unsafe_code)]

use serde::Deserialize;
use std::fs;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub phone: String,
    pub name: String,
}

pub fn load_contacts(path: &str) -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    
    let contacts: Vec<Contact> = serde_json::from_str(&content)?;

    Ok(contacts)
}

#[derive(Debug, Default)]
pub struct Node {
    children: BTreeMap<char, Node>,
    value: Option<String>,
}

impl Node {
    pub fn insert(&mut self, phone: &str, value: &str) {
        let mut current = self;

        for c in phone.chars() {
            current = current.children.entry(c).or_default();
        }

        current.value = Some(value.to_string());
    }

    pub fn display(&self, depth: usize) {
        for (c, children) in &self.children {
            println!("{} {}", "*".repeat(depth), c);

            if let Some(v) = &children.value {
                println!("{} {}", "*".repeat(depth + 1), v);
            }

            children.display(depth + 1);
        }
    }
}