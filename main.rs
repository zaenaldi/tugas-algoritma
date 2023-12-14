use std::collections::HashMap;
use std::io::{self, Write};

// Struct untuk merepresentasikan jenis barang
#[derive(Debug)]
struct JenisBarang {
    nama: String,
    deskripsi: String,
}

// Struct untuk merepresentasikan barang
#[derive(Debug)]
struct Barang {
    id: u32,
    nama: String,
    jenis: JenisBarang,
}

// Struct untuk menyimpan koleksi barang
struct KoleksiBarang {
    barang: HashMap<u32, Barang>,
}

impl KoleksiBarang {
    // Fungsi untuk menambahkan jenis barang baru
    fn tambah_jenis_barang(&mut self, id: u32, nama: &str, deskripsi: &str) {
        let jenis_barang = JenisBarang {
            nama: String::from(nama),
            deskripsi: String::from(deskripsi),
        };
        self.barang.insert(id, Barang {
            id,
            nama: String::from(""),
            jenis: jenis_barang,
        });
    }

    // Fungsi untuk menampilkan semua barang
    fn lihat_barang(&self) {
        for (_, barang) in &self.barang {
            println!("{:#?}", barang);
        }
    }

    // Fungsi untuk mengedit barang berdasarkan ID
    fn edit_barang(&mut self, id: u32, nama_baru: &str) {
        if let Some(barang) = self.barang.get_mut(&id) {
            barang.nama = String::from(nama_baru);
        } else {
            println!("Barang dengan ID {} tidak ditemukan.", id);
        }
    }

    // Fungsi untuk menghapus barang berdasarkan ID
    fn hapus_barang(&mut self, id: u32) {
        self.barang.remove(&id);
    }
}

fn main() {
    // Inisialisasi koleksi barang
    let mut koleksi_barang = KoleksiBarang { barang: HashMap::new() };

    loop {
        println!("Menu:");
        println!("1. Tambah Barang");
        println!("2. Lihat Barang");
        println!("3. Edit Barang");
        println!("4. Hapus Barang");
        println!("5. Keluar");

        print!("Pilih menu: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(choice) => {
                match choice {
                    1 => {
                        print!("ID Barang: ");
                        io::stdout().flush().unwrap();
                        let id: u32 = input_id();

                        print!("Nama Barang: ");
                        io::stdout().flush().unwrap();
                        let nama = input_string();

                        print!(" Jenis Barang: ");
                        io::stdout().flush().unwrap();
                        let deskripsi = input_string();

                        koleksi_barang.tambah_jenis_barang(id, &nama, &deskripsi);
                        println!("Jenis barang berhasil ditambahkan.");
                    }
                    2 => {
                        println!("Daftar Barang:");
                        koleksi_barang.lihat_barang();
                    }
                    3 => {
                        print!("ID Barang yang akan diubah: ");
                        io::stdout().flush().unwrap();
                        let id: u32 = input_id();

                        print!("Nama baru: ");
                        io::stdout().flush().unwrap();
                        let nama_baru = input_string();

                        koleksi_barang.edit_barang(id, &nama_baru);
                        println!("Barang berhasil diubah.");
                    }
                    4 => {
                        print!("ID Barang yang akan dihapus: ");
                        io::stdout().flush().unwrap();
                        let id: u32 = input_id();

                        koleksi_barang.hapus_barang(id);
                        println!("Barang berhasil dihapus.");
                    }
                    5 => {
                        println!("Keluar dari aplikasi.");
                        break;
                    }
                    _ => {
                        println!("Pilihan tidak valid.");
                    }
                }
            }
            Err(_) => {
                println!("Masukkan angka sebagai pilihan.");
            }
        }
    }
}

// Fungsi untuk input ID
fn input_id() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Input harus berupa angka")
}

// Fungsi untuk input string
fn input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}