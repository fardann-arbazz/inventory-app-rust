use std::usize;

use crate::errors::inventory_error::InventoryError;
use crate::models::items::Items;
use crate::utils::serde_file_inventory;

pub struct InventoryService {
    pub items: Vec<Items>,
    next_id: u32,
}

impl InventoryService {
    // function untuk menampilkan daftar barang serde file
    pub fn load_items_file() -> Self {
        if let Ok(items) = serde_file_inventory::load_items() {
            let next_id = items.last().map_or(1, |item| item.id + 1);

            Self { items, next_id }
        } else {
            Self {
                items: Vec::new(),
                next_id: 1,
            }
        }
    }

    // function untuk menambahkan barang
    pub fn add_barang(
        &mut self,
        name: String,
        stock: u32,
        price: f64,
    ) -> Result<(), InventoryError> {
        let items = Items {
            id: self.next_id,
            name,
            stock,
            price,
        };

        self.items.push(items);
        self.next_id += 1;

        // serde file inventory atau menyimpan data ke file
        serde_file_inventory::save_items(&self.items).map_err(|_| InventoryError::SaveError)?;

        Ok(())
    }

    //function untuk mengurangi stok barang
    pub fn reduce_stock(&mut self, item_id: u32, quantity: u32) -> Result<(), InventoryError> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.id == item_id)
            .ok_or(InventoryError::NotFound)?;

        if item.stock < quantity {
            return Err(InventoryError::InsufficientStock);
        } else {
            item.stock -= quantity;
            println!("Stok barang berhasil dikurangi.");

            // serde file inventory atau menyimpan data ke file
            serde_file_inventory::save_items(&self.items).map_err(|_| InventoryError::SaveError)?;
        }

        Ok(())
    }

    //function untuk menghapus barang
    pub fn delete_barang(&mut self, id: u32) {
        if let Some(index) = self.get_checked(id) {
            self.items.remove(index);
            println!("\n [SUCCESS] Barang berhasil di hapus.");

            // serde file inventory atau menyimpan data ke file
            let _ = serde_file_inventory::save_items(&self.items)
                .map_err(|_| InventoryError::SaveError);
        }
    }

    // function update barang
    pub fn update_barang(&mut self, id: u32, new_name: String, new_stock: u32, new_price: f64) {
        if let Some(index) = self.get_checked(id) {
            self.items[index].name = new_name;
            self.items[index].stock = new_stock;
            self.items[index].price = new_price;
            println!("\n [SUCCESS] Barang berhasil di update.");

            // serde file inventory atau menyimpan data ke file
            let _ = serde_file_inventory::save_items(&self.items)
                .map_err(|_| InventoryError::SaveError);
        }
    }

    // function update stock barang
    pub fn update_stock(&mut self, id: u32, new_stock: u32) {
        if let Some(index) = self.get_checked(id) {
            self.items[index].stock = new_stock;
            println!("\n [SUCCESS] Stok barang berhasil di update.");

            // serde file inventory atau menyimpan data ke file
            let _ =
                serde_file_inventory::save_items(&self.items).map(|_| InventoryError::SaveError);
        }
    }

    // function untuk search
    pub fn search_barang(&mut self, name: &String) -> Vec<Items> {
        self.items
            .iter()
            .filter(|item| item.name.to_lowercase().contains(&name.to_lowercase()))
            .cloned()
            .collect()
    }

    // function untuk format price
    pub fn format_price(price: f64) -> String {
        let mut results = price.to_string();
        let mut chars: Vec<char> = results.chars().collect();

        if chars.len() > 3 {
            let mut i = chars.len() - 3;
            while i > 0 {
                chars.insert(i, '.');
                if i > 3 {
                    i -= 3;
                } else {
                    break;
                }
            }

            results = chars.into_iter().collect();
        }

        results
    }

    // function untuk validasi index barang
    fn get_checked(&mut self, id: u32) -> Option<usize> {
        let index = self.items.iter().position(|item| item.id == id);

        if index.is_none() {
            println!("\n   [ERROR] Nomor barang tidak valid!");
        }

        index
    }
}
