use std::collections::HashMap;

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

    // function untuk menampilkan transaksi berdasarkan barang terlaris
    pub fn view_top_selling_items(&self) -> Vec<(String, u32)> {
        let mut item_counts: HashMap<String, u32> = HashMap::new();

        for tx in &self.records {
            let count = item_counts.entry(tx.item_name.clone()).or_insert(0);
            *count += tx.quantity;
        }

        let mut items: Vec<(String, u32)> = item_counts.into_iter().collect();

        items.sort_by(|a, b| b.1.cmp(&a.1));

        items
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
