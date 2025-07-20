use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TransactionError {
    NotFound,
    InvalidInput(String),
    InsufficientStock,
}

impl Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::NotFound => write!(f, "Item tidak ditemukan"),
            TransactionError::InvalidInput(msg) => write!(f, "Input tidak valid: {}", msg),
            TransactionError::InsufficientStock => write!(f, "Stok tidak mencukupi"),
        }
    }
}
