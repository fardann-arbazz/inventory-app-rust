use std::io::{self, Write};

use crate::errors::inventory_error::InventoryError;

// function untuk input index barang
pub fn get_index_barang() -> Result<u32, InventoryError> {
    print!("Masukan Nomor Barang: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| InventoryError::InvalidInput("Input tidak valid".to_string()))?;

    let index = input
        .trim()
        .parse::<u32>()
        .map_err(|_| InventoryError::InvalidInput("Nomor barang tidak valid".to_string()))?;

    Ok(index)
}

//function untuk input nama barang
pub fn get_name() -> Result<String, InventoryError> {
    print!("Masukan nama barang: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| InventoryError::InvalidInput("Gagal membaca input nama".to_string()))?;

    let name = input.trim().to_string();
    if name.is_empty() {
        Err(InventoryError::InvalidInput(
            "Nama tidak boleh kosong".to_string(),
        ))
    } else {
        Ok(name)
    }
}

// function untuk input stock
pub fn get_stock() -> Result<u32, InventoryError> {
    print!("Masukan stok barang: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| InventoryError::InvalidInput("Gagal membaca input stok".to_string()))?;

    let stock = input
        .trim()
        .parse::<u32>()
        .map_err(|_| InventoryError::InvalidInput("Stok harus berupa angka".to_string()))?;

    if stock <= 0 {
        return Err(InventoryError::InvalidInput(
            "Jumlah harus lebih dari 0".to_string(),
        ));
    }

    Ok(stock)
}

//function untuk input price
pub fn get_price() -> Result<f64, InventoryError> {
    print!("Masukan harga barang: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| InventoryError::InvalidInput("Gagal membaca input harga".to_string()))?;

    let price = input
        .trim()
        .replace(",", "")
        .parse()
        .map_err(|_| InventoryError::InvalidInput("Harga harus berupa angka".to_string()))?;

    Ok(price)
}
