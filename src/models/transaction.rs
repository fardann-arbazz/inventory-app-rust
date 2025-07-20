use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u32,
    pub item_id: u32,
    pub item_name: String,
    pub quantity: u32,
    pub total_price: f64,
    pub timestamp: NaiveDateTime,
}

impl Transaction {
    pub fn new(id: u32, item_id: u32, item_name: String, quantity: u32, total_price: f64) -> Self {
        Transaction {
            id,
            item_id,
            item_name,
            quantity,
            total_price,
            timestamp: Local::now().naive_local(),
        }
    }
}
