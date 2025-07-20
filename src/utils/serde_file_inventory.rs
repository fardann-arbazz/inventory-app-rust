use std::{
    error::Error,
    fs::{read_to_string, write},
};

use crate::models::items::Items;

pub fn save_items(items: &Vec<Items>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(items)?;
    write("items.json", json)?;
    Ok(())
}

pub fn load_items() -> Result<Vec<Items>, Box<dyn Error>> {
    let json = read_to_string("items.json")?;
    let items: Vec<Items> = serde_json::from_str(&json)?;

    Ok(items)
}
