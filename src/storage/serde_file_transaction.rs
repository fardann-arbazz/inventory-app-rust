use std::{
    error::Error,
    fs::{read_to_string, write},
};

use crate::models::transaction::Transaction;

pub fn save_items(items: &Vec<Transaction>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(items)?;
    write("transaction.json", json)?;

    Ok(())
}

pub fn load_items() -> Result<Vec<Transaction>, Box<dyn Error>> {
    let json = read_to_string("transaction.json")?;
    let items: Vec<Transaction> = serde_json::from_str(&json)?;

    Ok(items)
}
