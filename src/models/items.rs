use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Items {
    pub id: u32,
    pub name: String,
    pub stock: u32,
    pub price: f64,
}
