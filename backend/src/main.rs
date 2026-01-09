fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
}

fn first_word(s: &String) -> usize {
    // -> usize artinya mengembalikan nilai usize
    let bytes = s.as_bytes(); // Mengubah string menjadi array byte

    println!("ini data bytes: {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        // enumerate mengembalikan tuple (index, value)
        if item == b'd' {
            // b' ' adalah byte literal
            return i; // Mengembalikan index pertama yang ditemukan
        }
    }

    s.len() // Jika tidak ditemukan spasi, mengembalikan panjang string
}
