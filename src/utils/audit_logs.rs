use std::{
    fs::{OpenOptions, read_to_string},
    io::Write,
};

use chrono::Local;

// function untuk menambahkan log
pub fn add_logs(username: &str, action: &str) {
    let now = Local::now();
    let log_entry = format!("{} - {} melakukan: {}\n", now, username, action);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("audit.log")
        .expect("Gagal menyimpan log");

    if let Err(e) = file.write_all(log_entry.as_bytes()) {
        eprintln!("Gagal menulis log: {}", e);
    }
}

pub fn show_audit_logs() {
    match read_to_string("audit.log") {
        Ok(logs) => println!("{}", logs),
        Err(_) => println!("Tidak ada log yang ditemukan."),
    }

    println!("\nTekan Enter untuk kembali...");
    let mut pause = String::new();
    std::io::stdin().read_line(&mut pause).unwrap();
}
