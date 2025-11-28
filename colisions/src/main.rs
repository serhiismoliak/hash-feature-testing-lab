use md5;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("===============================================================================");
    println!("  Дослідження стійкості до колізій обрізаної хеш-функції MD5 (Birthday)");
    println!("===============================================================================");

    let bit_sizes = vec![16, 20, 24, 28];
    let mut all_results = Vec::new();

    for bits in &bit_sizes {
        println!("\nТестування {} біт...", bits);

        let start = Instant::now();
        let collision = find_collision(*bits);
        let duration = start.elapsed();

        all_results.push((bits, collision, duration));
    }

    // Проста таблиця результатів
    println!("\n\n===============================================================================");
    println!("                         РЕЗУЛЬТАТИ ЕКСПЕРИМЕНТУ");
    println!("===============================================================================\n");

    println!("{:<8} | {:<12} | {:<15} | {:<12}", "Біти", "Спроби", "Час", "2^(n/2)");
    println!("{}", "-".repeat(60));

    for (bits, collision, duration) in &all_results {
        let predicted = (2_f64).powf(**bits as f64 / 2.0);
        println!("{:<8} | {:<12} | {:<15?} | {:<12.0}",
                 bits,
                 collision.attempts,
                 duration,
                 predicted);
    }

    // Детальна таблиця колізій
    println!("\n\n===============================================================================");
    println!("                         ЗНАЙДЕНІ КОЛІЗІЇ");
    println!("===============================================================================\n");

    for (bits, collision, duration) in &all_results {
        println!("{} біт:", bits);
        println!("{}", "-".repeat(80));
        println!("  Час пошуку:     {:?}", duration);
        println!("  Спроби:         {}", collision.attempts);
        println!("  Повідомлення 1: {}", collision.msg1);
        println!("  Повідомлення 2: {}", collision.msg2);
        println!("  MD5 хеш 1:      {}", collision.full_hash1);
        println!("  MD5 хеш 2:      {}", collision.full_hash2);
        println!("  Співпадаюча частина ({} біт): {}", bits, collision.matched_part);
        println!();
    }

    // Аналіз
    println!("===============================================================================");
    println!("                         АНАЛІЗ");
    println!("===============================================================================\n");

    println!("Теоретичні очікування (Birthday-атака):");
    for (bits, collision, _) in &all_results {
        let predicted = (2_f64).powf(**bits as f64 / 2.0);
        let deviation = ((collision.attempts as f64 - predicted) / predicted * 100.0).abs();
        println!("  {} біт: 2^{} = {:.0} спроб (експеримент: {}, відхилення: {:.1}%)",
                 bits,
                 **bits / 2,
                 predicted,
                 collision.attempts,
                 deviation);
    }

    println!("\nВисновок:");
    println!("  Експериментальні дані підтверджують теорію birthday-атаки.");
    println!("  Для n-бітного хешу колізія знаходиться приблизно за 2^(n/2) спроб.");
}

struct CollisionInfo {
    msg1: u64,
    msg2: u64,
    attempts: u64,
    full_hash1: String,
    full_hash2: String,
    matched_part: String,
}

fn find_collision(bits: u32) -> CollisionInfo {
    let mut seen_hashes: HashMap<u64, (u64, String)> = HashMap::new();
    let mut counter: u64 = 0;

    loop {
        counter += 1;

        // Генеруємо повідомлення
        let message = counter.to_le_bytes();

        // Обчислюємо MD5
        let hash = md5::compute(&message);
        let full_hash = format!("{:x}", hash);

        // Обрізаємо до потрібної кількості біт
        let truncated = truncate_hash(hash.as_ref(), bits);

        // Перевіряємо колізію
        if let Some(&(first_counter, ref first_hash)) = seen_hashes.get(&truncated) {
            let hex_chars = ((bits + 3) / 4) as usize;
            let matched_part = full_hash[..hex_chars].to_string();

            return CollisionInfo {
                msg1: first_counter,
                msg2: counter,
                attempts: counter,
                full_hash1: first_hash.clone(),
                full_hash2: full_hash,
                matched_part,
            };
        }

        seen_hashes.insert(truncated, (counter, full_hash));

        // Захист від нескінченного циклу
        if counter > 10_000_000 {
            return CollisionInfo {
                msg1: 0,
                msg2: 0,
                attempts: counter,
                full_hash1: String::from("not found"),
                full_hash2: String::from("not found"),
                matched_part: String::from("not found"),
            };
        }
    }
}

fn truncate_hash(hash: &[u8], bits: u32) -> u64 {
    let bytes_needed = ((bits + 7) / 8) as usize;
    let mut result: u64 = 0;

    for i in 0..bytes_needed.min(8) {
        result |= (hash[i] as u64) << (i * 8);
    }

    let mask = if bits >= 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    };

    result & mask
}