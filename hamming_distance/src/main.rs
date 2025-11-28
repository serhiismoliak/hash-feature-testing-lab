use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Digest as Sha256Digest};
use sha3::{Sha3_256, Digest as Sha3Digest};

fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x ^ y).count_ones())
        .sum()
}
fn closeness_to_50_percent(distance: u32, digest_bits: u32) -> i32 {
    let ideal = (digest_bits / 2) as i32;
    (distance as i32 - ideal).abs()
}

fn percentage(distance: u32, digest_bits: u32) -> f32 {
    100.0 * (distance as f32) / digest_bits as f32
}

fn main() {
    let digest_md5_bits = (size_of::<md5::Digest>() * 8) as u32; // 128
    let digest_crc32_bits = (size_of::<u32>() * 8) as u32;     // 32

    let message1 = "hello world";
    let message2 = "hello wurld";

    let hash_crc1 = crc32fast::hash(message1.as_bytes());
    let hash2_crc2 = crc32fast::hash(message2.as_bytes());
    let hash_md5_1 = md5::compute(message1);
    let hash_md5_2 = md5::compute(message2);
    let hamming_distance_crc = hamming_distance(&hash_crc1.to_be_bytes(), &hash2_crc2.to_be_bytes());
    let hamming_distance_md5 = hamming_distance(&hash_md5_1.0, &hash_md5_2.0);

    println!("Дистанція Хемінга між хешем слів '{message1}' та '{message2}'\n");

    println!("+--------+---------------+--------------+-------------------+-------------------------+");
    println!("| Алг-рт | Змінені біти  | Розмір хешу  | % змінених бітів  | Відхилення від 50%      |");
    println!("+--------+---------------+--------------+-------------------+-------------------------+");

    println!(
        "| MD5    | {:>13} | {:>12} | {:>16.2}% | {:>22}% |",
        hamming_distance_md5,
        digest_md5_bits,
        percentage(hamming_distance_md5, digest_md5_bits),
        closeness_to_50_percent(hamming_distance_md5, digest_md5_bits)
    );

    println!(
        "| CRC32  | {:>13} | {:>12} | {:>16.2}% | {:>22}% |",
        hamming_distance_crc,
        digest_crc32_bits,
        percentage(hamming_distance_crc, digest_crc32_bits),
        closeness_to_50_percent(hamming_distance_crc, digest_crc32_bits)
    );

    println!("+--------+---------------+--------------+-------------------+-------------------------+");
}
