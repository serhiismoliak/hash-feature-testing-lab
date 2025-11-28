use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    let file = r"C:\Users\user\Downloads\openssl-3.0.18.tar.gz";

    println!("Перевірка: {}", file);

    let expected = match std::fs::read_to_string(format!("{}.sha256", file)) {
        Ok(content) => content.split_whitespace().next().unwrap().to_string(),
        Err(_) => {
            println!("Помилка: файл {}.sha256 не знайдено", file);
            return;
        }
    };

    let mut f = BufReader::new(File::open(file).unwrap());
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 4096];

    loop {
        let n = f.read(&mut buf).unwrap();
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }

    let computed = format!("{}", hex::encode(hasher.finalize()));

    println!("Обчислений: {}", computed);
    println!("Очікуваний: {}", expected);
    println!();

    if computed == expected {
        println!("OK - Хеші співпадають!");
    } else {
        println!("FAIL - Хеші НЕ співпадають!");
    }
}
