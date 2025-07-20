use std::io::{self, Write};

use crate::{
    handlers::{inventory_handlers, transaction_handlers, users_handlers},
    models::users::{Role, User},
    services::{inventory::InventoryService, transaction::TransactionService, users::UserService},
    utils::audit_logs,
};

const TOTAL_MENU_ADMIN: u32 = 15;
const TOTAL_MENU_KASIR: u32 = 6;

#[allow(dead_code)]
pub enum MenuExitStatus {
    Logout,
    Exit,
}

/// Main menu loop function
pub fn run(user_service: &mut UserService) -> Result<MenuExitStatus, Box<dyn std::error::Error>> {
    // Initialize serde file json
    let mut inventory = InventoryService::load_items_file();
    let mut transaction = TransactionService::new();

    // Welcome and login
    print_welcome_message();

    let user = authenticate_user(user_service)?;
    println!(
        "✅ Login berhasil sebagai: {} ({:?})",
        user.username, user.role
    );

    // Menunggu users enter untuk melanjutkan
    wait_for_user_input("Tekan Enter untuk melanjutkan...");

    // Main application loop
    let exit_status = run_main_menu(user, user_service, &mut inventory, &mut transaction)?;

    Ok(exit_status)
}

/// Function to print welcome message
fn print_welcome_message() {
    println!("========================================");
    println!("== Selamat Datang di Sistem Inventory ==");
    println!("========================================");
    println!("Silakan login terlebih dahulu.\n");
}

/// Handles user authentication
fn authenticate_user(user_service: &mut UserService) -> Result<User, Box<dyn std::error::Error>> {
    match users_handlers::handle_login(user_service) {
        Ok(()) => {
            let user = user_service.get_current_user().unwrap();
            Ok(user)
        }
        Err(e) => {
            println!("❌ Gagal login: {}", e);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Login gagal",
            )))
        }
    }
}

/// Jalankan menu utama aplikasi
fn run_main_menu(
    user: User,
    user_service: &mut UserService,
    inventory: &mut InventoryService,
    transaction: &mut TransactionService,
) -> Result<MenuExitStatus, Box<dyn std::error::Error>> {
    loop {
        display_menu(&user);

        let choice = get_user_input("")?;

        // Handle menu choice dan cek return value
        match handle_menu_choice(&choice, &user, user_service, inventory, transaction)? {
            Some(MenuExitStatus::Logout) => {
                println!("🔒 Logout berhasil! Kembali ke menu login...");
                return Ok(MenuExitStatus::Logout);
            }
            Some(MenuExitStatus::Exit) => {
                println!("👋 Keluar dari sistem. Sampai jumpa!");
                return Ok(MenuExitStatus::Exit);
            }
            None => {
                // Continue dengan menu
                if choice.trim() != "0" && choice.trim() != "16" {
                    wait_for_user_input("\nTekan Enter untuk kembali ke menu...");
                }
            }
        }
    }
}

/// Menampilkan menu yang sesuai berdasarkan peran pengguna
fn display_menu(user: &User) {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen

    // Header
    println!("\n\x1B[1;34m╔════════════════════════════════════════════╗");
    println!("║           INVENTORY MANAGEMENT SYSTEM      ║");
    println!("╠════════════════════════════════════════════╣");
    println!("║ Role: {: <36} ║", format!("{:?}", user.role));
    println!("╚════════════════════════════════════════════╝\x1B[0m");

    // CORE MENU (for all users)
    println!("\n\x1B[1;36mCORE OPERATIONS:\x1B[0m");
    println!("  \x1B[32m1.\x1B[0m Transaksi Baru");
    println!("  \x1B[32m2.\x1B[0m Riwayat Transaksi");
    println!("  \x1B[32m3.\x1B[0m Laporan Transaksi");
    println!("  \x1B[32m4.\x1B[0m Cari Transaksi (Tanggal)");
    println!("  \x1B[32m5.\x1B[0m Lihat Stok Barang");

    if is_admin(user) {
        // INVENTORY MANAGEMENT (Admin only)
        println!("\n\x1B[1;36mINVENTORY MANAGEMENT:\x1B[0m");
        println!("  \x1B[33m6.\x1B[0m Tambah Barang");
        println!("  \x1B[33m7.\x1B[0m Hapus Barang");
        println!("  \x1B[33m8.\x1B[0m Update Barang");
        println!("  \x1B[33m9.\x1B[0m Update Stok");
        println!("  \x1B[33m10.\x1B[0m Cari Barang");

        // USER MANAGEMENT (Admin only)
        println!("\n\x1B[1;36mUSER MANAGEMENT:\x1B[0m");
        println!("  \x1B[35m11.\x1B[0m Lihat User");
        println!("  \x1B[35m12.\x1B[0m Tambah User");
        println!("  \x1B[35m13.\x1B[0m Hapus User");
        println!("  \x1B[35m14.\x1B[0m Update User");
        println!("  \x1B[35m15.\x1B[0m Audit Logs");
    }

    // SYSTEM ACTIONS (for all users)
    println!("\n\x1B[1;36mSYSTEM ACTIONS:\x1B[0m");
    println!("  \x1B[31mL.\x1B[0m Logout");
    println!("  \x1B[31m0.\x1B[0m Keluar");

    // Input prompt
    print!(
        "\n\x1B[1mPilih menu [{}]: \x1B[0m",
        if is_admin(user) {
            "1-15, L, 0"
        } else {
            "1-5, L, 0"
        }
    );
    let _ = std::io::stdout().flush();
}
/// Menampilkan pilihan menu dan mengambil input dari pengguna
fn get_user_input(prompt: &str) -> Result<String, std::io::Error> {
    if !prompt.is_empty() {
        print!("{}", prompt);
        io::stdout().flush()?;
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// Function untuk menunggu input dari pengguna
fn wait_for_user_input(prompt: &str) {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
}

/// Fungsi untuk menangani pilihan menu yang dipilih oleh pengguna
fn handle_menu_choice(
    choice: &str,
    user: &User,
    users_service: &mut UserService,
    inventory: &mut InventoryService,
    transaction: &mut TransactionService,
) -> Result<Option<MenuExitStatus>, Box<dyn std::error::Error>> {
    println!();

    match choice.trim() {
        // Kasir dan Admin
        "1" => {
            println!("🔄 Memulai transaksi...\n");
            match transaction_handlers::make_transaction(transaction, inventory, users_service) {
                Ok(_) => {
                    println!("\n✅ Transaksi berhasil diselesaikan!");
                }
                Err(e) => {
                    println!("\n❌ Transaksi gagal: {}", e);

                    // Additional specific error handling
                    let error_msg = format!("{}", e);
                    if error_msg.contains("kosong") || error_msg.contains("empty") {
                        println!(
                            "💡 Tip: Pastikan barang yang ingin dijual tersedia di inventori."
                        );
                    } else if error_msg.contains("stok") || error_msg.contains("stock") {
                        println!("💡 Tip: Periksa stok barang sebelum melakukan transaksi.");
                    }
                }
            }
            Ok(None)
        }
        "2" => {
            println!("📋 Menampilkan riwayat transaksi...\n");
            transaction_handlers::view_records(transaction);
            Ok(None)
        }
        "3" => {
            println!("📊 Menampilkan laporan transaksi...\n");
            transaction_handlers::view_total_transaction(transaction);
            Ok(None)
        }
        "4" => {
            println!("📅 Mencari transaksi berdasarkan tanggal...\n");
            transaction_handlers::view_transaction_by_date(transaction);
            Ok(None)
        }
        "5" => {
            println!("📦 Menampilkan stok inventori...\n");
            inventory_handlers::view_items(inventory);
            Ok(None)
        }

        // Admin only
        "6" if is_admin(user) => {
            println!("➕ Menambah barang baru...\n");
            match inventory_handlers::add_items(inventory) {
                Ok(_) => {
                    println!("\n✅ Barang berhasil ditambahkan!");
                }
                Err(e) => {
                    println!("\n❌ Gagal menambah barang: {}", e);
                }
            }
            Ok(None)
        }
        "7" if is_admin(user) => {
            println!("🗑️ Menghapus barang...\n");
            match inventory_handlers::handle_delete_barang(inventory) {
                Ok(_) => {
                    println!("\n✅ Barang berhasil dihapus!");
                }
                Err(e) => {
                    println!("\n❌ Gagal menghapus barang: {}", e);
                }
            }
            Ok(None)
        }
        "8" if is_admin(user) => {
            println!("✏️ Mengupdate barang...\n");
            match inventory_handlers::handle_update_barang(inventory) {
                Ok(_) => {
                    println!("\n✅ Barang berhasil diupdate!");
                }
                Err(e) => {
                    println!("\n❌ Gagal mengupdate barang: {}", e);
                }
            }
            Ok(None)
        }
        "9" if is_admin(user) => {
            println!("📊 Mengupdate stok barang...\n");
            match inventory_handlers::handle_update_stock(inventory) {
                Ok(_) => {
                    println!("\n✅ Stok barang berhasil diupdate!");
                }
                Err(e) => {
                    println!("\n❌ Gagal mengupdate stok: {}", e);
                }
            }
            Ok(None)
        }
        "10" if is_admin(user) => {
            println!("🔍 Mencari barang...\n");
            match inventory_handlers::handle_search_barang(inventory) {
                Ok(_) => {
                    println!("\n✅ Pencarian selesai!");
                }
                Err(e) => {
                    println!("\n❌ Gagal mencari barang: {}", e);
                }
            }
            Ok(None)
        }
        "11" if is_admin(user) => {
            println!("📋 Menampilkan Users...\n");
            users_handlers::handle_users_list(users_service);
            Ok(None)
        }

        "12" if is_admin(user) => {
            println!("➕ Menambah users baru...\n");
            match users_handlers::handle_users(users_service) {
                Ok(_) => {
                    println!("\n✅ Tambah users selesai!");
                }
                Err(e) => {
                    println!("\n❌ Gagal menambah users: {}", e);
                }
            }
            Ok(None)
        }

        "13" if is_admin(user) => {
            println!("➖ Hapus users baru...\n");
            match users_handlers::handle_delete_users(users_service) {
                Ok(_) => {
                    println!("\n✅ Hapus users selesai!");
                }
                Err(e) => {
                    println!("\n❌ Gagal hapus users: {}", e);
                }
            }
            Ok(None)
        }

        "14" if is_admin(user) => {
            println!("✏️ Update users baru...\n");
            match users_handlers::handle_update_users(users_service) {
                Ok(_) => {
                    println!("\n✅ Update users selesai!");
                }
                Err(e) => {
                    println!("\n❌ Gagal update users: {}", e);
                }
            }
            Ok(None)
        }

        "15" if is_admin(user) => {
            println!("📝 Menampilkan logs...\n");
            audit_logs::show_audit_logs();
            Ok(None)
        }

        "L" => {
            println!("🔒 Memproses logout...");
            match users_handlers::handle_logout(users_service) {
                Ok(status) => Ok(Some(status)),
                Err(e) => {
                    println!("❌ Gagal logout: {}", e);
                    Ok(None)
                }
            }
        }

        "0" => {
            println!("🚪 Keluar dari sistem...");
            Ok(Some(MenuExitStatus::Exit))
        }

        // handle choice invalid
        choice if choice.parse::<u32>().is_ok() => {
            let num = choice.parse::<u32>().unwrap();
            if num >= TOTAL_MENU_KASIR && num <= TOTAL_MENU_ADMIN && !is_admin(user) {
                println!("❌ Akses ditolak: Fitur ini hanya tersedia untuk Admin.");
                println!("💡 Login sebagai Admin untuk mengakses fitur manajemen inventori.");
            } else {
                println!("❌ Pilihan menu tidak tersedia: {}", choice);
                println!("💡 Pilih menu yang tersedia sesuai dengan role Anda.");
            }
            Ok(None)
        }

        _ => {
            println!("❌ Input tidak valid: '{}'", choice);
            println!("💡 Masukkan angka sesuai dengan menu yang tersedia.");
            Ok(None)
        }
    }
}

/// Check jika users adalah admin
fn is_admin(user: &User) -> bool {
    matches!(user.role, Role::Admin)
}
