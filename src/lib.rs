#![forbid(unsafe_code)]

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Contact {
    phone: String,
    name: String,
}

pub fn load_contacts(url: &str) -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(url)?;
    let contacts: Vec<Contact> = serde_json::from_str(&content)?;
    Ok(contacts)
}