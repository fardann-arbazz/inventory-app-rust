use chrono::NaiveDate;

use crate::{models::transaction::Transaction, storage::serde_file_transaction};

pub struct TransactionService {
    pub records: Vec<Transaction>,
    pub next_id: u32,
}

impl TransactionService {
    // function untuk menampilkan transaksi serde file
    pub fn load_items_file() -> Self {
        if let Ok(transactions) = serde_file_transaction::load_items() {
            let next_id = transactions.last().map_or(1, |tx| tx.id + 1);

            Self {
                records: transactions,
                next_id,
            }
        } else {
            Self {
                records: Vec::new(),
                next_id: 1,
            }
        }
    }

    // function untuk menampilkan transaksi berdasarkan tanggal
    pub fn view_transactions_by_date(&self, date: NaiveDate) -> Vec<&Transaction> {
        self.records
            .iter()
            .filter_map(|tx| {
                if tx.timestamp.date() == date {
                    Some(tx)
                } else {
                    None
                }
            })
            .collect()
    }

    // function untuk menghitung total harga
    pub fn calculate_total_price(&self, price_per_item: f64, quantity: u32) -> f64 {
        let total = price_per_item * quantity as f64;
        if quantity > 10 {
            total * 0.9 //diskon 10%
        } else {
            total
        }
    }
}
