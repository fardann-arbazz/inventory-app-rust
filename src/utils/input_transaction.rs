use std::io::{self, Write};

use chrono::NaiveDate;

use crate::errors::transaction_error::TransactionError;

// function untuk mendapatkan input ID barang
pub fn get_input_item_id() -> Result<u32, TransactionError> {
    print!("Masukan ID barang: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| TransactionError::InvalidInput("Gagal membaca input ID barang".to_string()))?;

    let item_id = input
        .trim()
        .parse::<u32>()
        .map_err(|_| TransactionError::InvalidInput("ID barang harus berupa angka".to_string()))?;

    Ok(item_id)
}

// function untuk mendapatkan input jumlah barang
pub fn get_input_quantity() -> Result<u32, TransactionError> {
    print!("Masukan jumlah barang: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| {
        TransactionError::InvalidInput("Gagal membaca input jumlah barang".to_string())
    })?;

    let qty_input = input.trim().parse::<u32>().map_err(|_| {
        TransactionError::InvalidInput("Input jumlah harus berupa angka".to_string())
    })?;

    Ok(qty_input)
}

// function untuk mendapatkan input tanggal
pub fn get_input_date() -> Result<NaiveDate, TransactionError> {
    print!("Masukan tanggal transaksi (2025-01-01): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| TransactionError::InvalidInput("Gagal membaca input tanggal".to_string()))?;

    let date_input = input.trim().parse::<NaiveDate>().map_err(|_| {
        TransactionError::InvalidInput("Input tanggal harus berupa format YYYY-MM-DD".to_string())
    })?;

    Ok(date_input)
}
