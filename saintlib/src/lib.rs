// Import necessary libraries
use serde::Deserialize; // For converting JSON into Rust structs
use std::fs; // For reading files from disk

// Define the structure of a saint, matching the JSON fields
#[derive(Debug, Deserialize, Clone)]
pub struct Saint {
    pub month: u32,
    pub day: u32,
    pub name: String,
    pub description: String,
}

// Function to load the list of saints from a JSON file
pub fn load_saints_from_file(path: &str) -> Vec<Saint> {
    // Read the entire file into a String
    let data = fs::read_to_string(path).expect("Failed to read saints JSON file");
    // Convert the JSON string into a Vec<Saint>
    serde_json::from_str(&data).expect("Failed to parse saints JSON")
}

// Function to find saints for a given date (month + day)
pub fn saints_for_date(saints: &[Saint], month: u32, day: u32) -> Vec<Saint> {
    saints
        .iter() // Iterate over all saints
        .filter(|s| s.month == month && s.day == day) // Keep only those matching the date
        .cloned() // Clone them so we can return owned values
        .collect() // Collect into a Vec<Saint>
}
