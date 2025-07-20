use crate::{
    errors::inventory_error::InventoryError,
    services::inventory::InventoryService,
    utils::{audit_logs, input_inventory},
};

//function untuk menambahkan barqang
pub fn add_items(service: &mut InventoryService) -> Result<(), InventoryError> {
    println!();
    println!("====================");
    println!("== Tambah Barang ==");
    println!("====================");
    println!();

    let name = input_inventory::get_name()?;
    let stock = input_inventory::get_stock()?;
    let price = input_inventory::get_price()?;

    audit_logs::add_logs("admin", "Menambahkan barang");

    service.add_barang(name, stock, price)?;

    println!("Barang berhasil ditambahkan!");

    Ok(())
}

//function untuk menampilkan daftar barang
pub fn view_items(service: &InventoryService) {
    println!("\n== Daftar Barang ==");

    if service.items.is_empty() {
        println!("Tidak ada barang dalam inventori.");
    } else {
        for item in &service.items {
            if item.stock <= 5 {
                println!(
                    "[!] Peringatan: Stok '{}' tinggal {}!",
                    item.name, item.stock
                );
            }

            println!();

            println!(
                "[{}] {} | Stok: {} | Harga: Rp{:>6}",
                item.id,
                item.name,
                item.stock,
                InventoryService::format_price(item.price)
            );
        }
    }
}

// function untuk handle delete barang
pub fn handle_delete_barang(service: &mut InventoryService) -> Result<(), InventoryError> {
    println!("==============");
    println!("HAPUS BARANG");
    println!("==============");

    if service.items.is_empty() {
        println!("Tidak ada barang dalam inventori.");
        return Ok(());
    } else {
        view_items(&service);

        let index_input = input_inventory::get_index_barang()?;
        let barang_name = get_nama_barang(service, index_input).unwrap_or_default();

        service.delete_barang(index_input);

        let msg_logs = format!("Menghapus barang '{}'", barang_name);
        audit_logs::add_logs("admin", &msg_logs);

        Ok(())
    }
}

// function untuk handle update barang
pub fn handle_update_barang(service: &mut InventoryService) -> Result<(), InventoryError> {
    println!("==============");
    println!("UPDATE BARANG");
    println!("==============");

    if service.items.is_empty() {
        println!("Tidak ada barang dalam inventori.");
        return Ok(());
    } else {
        view_items(&service);

        let index_input = input_inventory::get_index_barang()?;
        let new_name = input_inventory::get_name()?;
        let new_stock = input_inventory::get_stock()?;
        let new_price = input_inventory::get_price()?;

        let msg_logs = format!("Mengupdate barang '{}'", new_name);
        service.update_barang(index_input, new_name, new_stock, new_price);

        audit_logs::add_logs("admin", &msg_logs);

        Ok(())
    }
}

// function untuk handle update stock barang
pub fn handle_update_stock(service: &mut InventoryService) -> Result<(), InventoryError> {
    println!("==============");
    println!("UPDATE STOCK");
    println!("==============");

    if service.items.is_empty() {
        println!("Tidak ada barang dalam inventori.");
        return Ok(());
    } else {
        view_items(&service);

        let index_input = input_inventory::get_index_barang()?;
        let new_stock = input_inventory::get_stock()?;

        let nama_barang = get_nama_barang(service, index_input).unwrap_or_default();
        service.update_stock(index_input, new_stock);

        let log_msg = format!("Mengupdate stok '{}' menjadi {}", nama_barang, new_stock);
        // Log update stok
        audit_logs::add_logs("admin", &log_msg);
        Ok(())
    }
}

// function untuk get nama barang berdasarkan id
pub fn get_nama_barang(service: &mut InventoryService, id: u32) -> Option<String> {
    service
        .items
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.name.clone())
}

// function untuk handle search barang
pub fn handle_search_barang(service: &mut InventoryService) -> Result<(), InventoryError> {
    println!("=============");
    println!("Cari Barang");
    println!("=============");

    let name_input = input_inventory::get_name()?;
    let items = service.search_barang(&name_input);

    if items.is_empty() {
        println!("Tidak ada barang ditemukan");
        println!("Kata kunci: '{}'", name_input);
    } else {
        println!("=================");
        println!("Hasil Pencarian");
        println!("=================");

        for (i, item) in items.iter().enumerate() {
            println!(
                "{}. {} | {} | Rp{}",
                i + 1,
                item.name,
                item.stock,
                InventoryService::format_price(item.price)
            );
        }
    }

    Ok(())
}
