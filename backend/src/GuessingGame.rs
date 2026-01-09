use rand::Rng;
use std::io;

fn main() {
    println!("Tebak angka dari 1 sampai 10!");

    // Generate secret dulu!
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=10);

    // Debug sementara (hapus nanti biar fair)
    // println!("Secret: {}", secret_number);

    loop {
        println!("Masukkan tebakanmu: 1 to 10");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Gagal membaca baris");

        // Parse ke u32, handle error graceful
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Harus angka valid! Coba lagi.");
                continue; // Ulang loop
            }
        };

        println!("Kamu tebak: {}", guess);

        // Bandingkan
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Terlalu kecil!"),
            std::cmp::Ordering::Greater => println!("Terlalu besar!"),
            std::cmp::Ordering::Equal => {
                println!("Benar! Secretnya {}", secret_number);
                break; // Keluar loop, game selesai
            }
        }
    }
}
