use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum InventoryError {
    NotFound,
    InvalidInput(String),
    InsufficientStock,
    SaveError,
}

impl Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::NotFound => write!(f, "Item tidak ditemukan"),
            InventoryError::InvalidInput(msg) => write!(f, "Input tidak valid: {}", msg),
            InventoryError::InsufficientStock => write!(f, "Stok tidak mencukupi"),
            InventoryError::SaveError => write!(f, "Gagal menyimpan data"),
        }
    }
}

impl Error for InventoryError {}
