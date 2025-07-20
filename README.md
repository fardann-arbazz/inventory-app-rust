# 🏪 Sistem Manajemen Inventori CLI (Rust)

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)
![Lisensi](https://img.shields.io/badge/Lisensi-MIT-blue)

Sistem **manajemen inventori berbasis command-line** yang dibangun dengan Rust, menampilkan akses multi-role, proses transaksi, pelaporan, dan penyimpanan data JSON yang persisten.

## ✨ Fitur Utama

### 📦 Fungsi Inti
- **Sistem multi-role** (Admin & Kasir)
- **Operasi CRUD inventori** dengan manajemen stok
- **Proses transaksi** dengan filter tanggal
- **Pelaporan lengkap** (penjualan, pendapatan, inventori)
- **Manajemen pengguna** (buat, update, hapus user)

### 🛠️ Keunggulan Teknis
- **Penyimpanan data** menggunakan file JSON
- **Validasi input** dan penanganan error
- **Antarmuka CLI berwarna** dengan ANSI escapes
- **Arsitektur modular** mengikuti best practice Rust

## 🧑‍💻 Peran Pengguna

| Peran  | Hak Akses                              |
|--------|----------------------------------------|
| Admin  | Akses penuh termasuk manajemen user    |
| Kasir  | Proses transaksi dan lihat laporan     |

## 🚀 Memulai

### Prasyarat
- Rust 1.70+ (disarankan: gunakan [rustup](https://rustup.rs/))

### Instalasi
```bash
git clone https://github.com/fardann-arbazz/inventory-app-rust
cd inventory-app-rust
cargo run
```

Sistem akan otomatis membuat file JSON yang diperlukan jika belum ada.

## 🖥️ Tampilan Sistem

### Menu Utama (Admin)
```text
╔════════════════════════════════════════════╗
║           SISTEM MANAJEMEN INVENTORI       ║
╠════════════════════════════════════════════╣
║ Role: Admin                                ║
╚════════════════════════════════════════════╝

OPERASI UTAMA:
  1. Transaksi Baru
  2. Riwayat Transaksi
  3. Laporan Penjualan
  4. Cari Transaksi Berdasarkan Tanggal
  5. Lihat Inventori

MANAJEMEN INVENTORI:
  6. Tambah Barang
  7. Hapus Barang
  8. Update Barang
  9. Update Stok
  10. Cari Barang

MANAJEMEN PENGGUNA:
  11. Daftar Pengguna
  12. Tambah Pengguna
  13. Hapus Pengguna
  14. Update Pengguna
  15. Lihat Logs

AKSI SISTEM:
  L. Logout
  0. Keluar
```

### Contoh Laporan
```text
=== LAPORAN PENJUALAN HARI INI ===
Total Transaksi : 12
Total Pendapatan : Rp. 1.240.500
Barang Terjual   : 47 unit
Barang Terlaris  : Notebook (15 unit)
```

## 📂 Struktur Data

### Contoh items.json
```json
[
  {
    "id": "ITEM-001",
    "nama": "Notebook Premium",
    "kategori": "Alat Tulis",
    "stok": 45,
    "harga": 12500.0,
    "terakhir_update": "2025-07-20T09:30:00Z"
  }
]
```

## 🛠️ Dibangun Dengan

- **Inti**: Rust Edisi 2021+
- **Serialisasi**: [serde](https://serde.rs/) + [serde_json](https://docs.rs/serde_json)
- **Tanggal/Waktu**: [chrono](https://docs.rs/chrono)
- **Gaya CLI**: Kode ANSI escape

## 📈 Rencana Pengembangan

### Fitur yang Direncanakan
- [ ] Keranjang belanja (transaksi multi-barang)
- [ ] Cetak struk transaksi
- [ ] Ekspor laporan ke CSV
- [ ] Pelacakan barang terlaris
- [ ] Kategori inventori
- [ ] Fungsi undo transaksi

### Kemungkinan Pengembangan
- [ ] Antarmuka web (Actix-Web/Axum)
- [ ] GUI desktop (Tauri/egui)
- [ ] Aplikasi mobile pendamping

## 🤝 Berkontribusi

Kontribusi diterima! Silakan ikuti langkah berikut:
1. Fork proyek ini
2. Buat branch fitur Anda (`git checkout -b fitur/FiturAnda`)
3. Commit perubahan Anda (`git commit -m 'Tambahkan FiturAnda'`)
4. Push ke branch (`git push origin fitur/FiturAnda`)
5. Buka Pull Request

## 📜 Lisensi

Didistribusikan di bawah lisensi MIT. Lihat `LICENSE` untuk informasi lebih lanjut.

## 📬 Kontak

Fardan Arbaz - [@frdn.arbzz](https://www.instagram.com/frdn.arbzz/) - fardanarbas5@contoh.com

Link Proyek: [https://github.com/fardann-arbazz/inventory-app-rust](https://github.com/fardann-arbazz/inventory-app-rust)
```
