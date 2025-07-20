mod errors;
mod handlers;
mod menu;
mod models;
mod services;
mod storage;
mod utils;

use crate::menu::{MenuExitStatus, run};
use crate::services::users::UserService;

fn main() {
    let mut user_service = UserService::new();

    loop {
        match run(&mut user_service) {
            Ok(MenuExitStatus::Logout) => {
                println!("🔁 Kembali ke menu login...\n");
                continue;
            }
            Ok(MenuExitStatus::Exit) => {
                println!("👋 Terima kasih telah menggunakan sistem inventory!");
                break; // Keluar dari aplikasi
            }
            Err(e) => {
                eprintln!("❌ Terjadi kesalahan: {}", e);
                println!("🔁 Kembali ke menu login...\n");
                continue;
            }
        }
    }
}
