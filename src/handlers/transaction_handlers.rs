use chrono::Local;

use crate::{
    errors::transaction_error::TransactionError,
    handlers::inventory_handlers,
    models::transaction::Transaction,
    services::{inventory::InventoryService, transaction::TransactionService, users::UserService},
    storage::serde_file_transaction,
    utils::{audit_logs, input_transaction},
};

const MAX_QUANTITY: u32 = 10;

// function untuk menambahkan transaksi
pub fn make_transaction(
    service: &mut TransactionService,
    inventory: &mut InventoryService,
    user_service: &mut UserService,
) -> Result<(), TransactionError> {
    if inventory.items.is_empty() {
        return Err(TransactionError::NotFound);
    }

    inventory_handlers::view_items(&inventory);
    println!();

    let item_id = input_transaction::get_input_item_id()?;
    let quantity = input_transaction::get_input_quantity()?;

    if let Err(e) = inventory.reduce_stock(item_id, quantity) {
        println!("Gagal melakukan transaksi: {}", e);
        return Ok(());
    }

    let Some(item) = inventory.items.iter().find(|i| i.id == item_id) else {
        println!("Item tidak ditemukan.");
        return Ok(());
    };

    let total_price = TransactionService::calculate_total_price(&service, item.price, quantity);
    let total_display = InventoryService::format_price(total_price);

    if quantity > MAX_QUANTITY {
        println!("Selamat! Anda mendapatkan diskon 10%");
    }

    let transaction = Transaction::new(
        service.next_id,
        item_id,
        item.name.clone(),
        quantity,
        total_price,
    );

    let msg_logs = format!(
        "Transaksi {} x{} = Rp{} | {}",
        item.name,
        quantity,
        total_display,
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    user_service.current_user = Some(user_service.users[0].clone());

    // Lalu baru kamu bisa panggil di tempat lain:
    if let Some(user) = user_service.get_current_user() {
        let username = user.username;
        audit_logs::add_logs(&username, &msg_logs);
    } else {
        println!("Tidak ada user yang login.");
    }

    // Tambahkan transaksi ke dalam serde file
    let _ = serde_file_transaction::save_items(&vec![transaction.clone()]);

    service.records.push(transaction);
    service.next_id += 1;

    println!("Transaksi berhasil!! Total: Rp{}", total_display);

    Ok(())
}

//function untuk menampilkan riwayat transaksi
pub fn view_records(service: &TransactionService) {
    println!("\n== Riwayat Transaksi ==");

    if service.records.is_empty() {
        println!("Tidak ada riwayat transaksi.");
        return;
    } else {
        for tx in &service.records {
            let format_price = InventoryService::format_price(tx.total_price);

            println!(
                "#{} | {} x{} = Rp{} | {}",
                tx.id, tx.item_name, tx.quantity, format_price, tx.timestamp
            );
        }
    }
}

// function untuk handle menampilkan transaksi berdasarkan barang terlaris
pub fn view_top_selling_item(service: &TransactionService) {
    println!("=========================");
    println!("\n== Barang Terlaris ==");
    println!("=========================");

    let items_transaction = service.view_top_selling_items();

    if items_transaction.is_empty() {
        println!("Tidak ada barang terlaris.");
        return;
    }

    for (item_name, count) in items_transaction {
        println!("{}: {} item", item_name, count);
    }
}

// function untuk menampilkan total transaksi hari ini
pub fn view_total_transaction(service: &TransactionService) {
    let today = Local::now().date_naive();

    let transaction_today: Vec<&Transaction> = service
        .records
        .iter()
        .filter(|tx| tx.timestamp.date() == today)
        .collect();

    let total_transaction: f64 = transaction_today.iter().map(|tx| tx.total_price).sum();
    let items_sold = transaction_today.len();
    let format_price = InventoryService::format_price(total_transaction);

    println!("Total transaksi hari ini: {}", format_price);
    println!("Total barang terjual: {} item", items_sold);
}

// function untuk menampilkan transaksi berdasarkan tanggal
pub fn view_transaction_by_date(service: &TransactionService) {
    let date = match input_transaction::get_input_date() {
        Ok(date) => date,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let filtered_transactions = service.view_transactions_by_date(date);

    if filtered_transactions.is_empty() {
        println!("Tidak ada transaksi pada tanggal tersebut.");
    } else {
        println!("\n===============================");
        println!("Transaksi pada tanggal: {}", date);
        println!("===============================\n");

        println!(
            "{:<5} | {:<20} | {:<8} | {:<12} | {:<20}",
            "ID", "Nama Item", "Jumlah", "Total Harga", "Waktu"
        );
        println!("{}", "-".repeat(75));

        for tx in &filtered_transactions {
            println!(
                "{:<5} | {:<20} | {:<8} | Rp{:<11.2} | {:<20}",
                tx.id,
                tx.item_name,
                tx.quantity,
                InventoryService::format_price(tx.total_price),
                tx.timestamp
            );
        }

        println!("\nTotal transaksi: {}", filtered_transactions.len());
    }
}
