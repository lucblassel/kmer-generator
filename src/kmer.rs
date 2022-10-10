
use rand::Rng;

/// Generate a random k-ner
pub fn generate(k: u8) -> String {
    let max_k: u64 = 1 << (2 * k);
    let mut rng = rand::thread_rng();
    let num: u64 = rng.gen_range(0..max_k);

    decode(&num, k)
}

/// Encode up to a 32-mer with 2-bit encoding:
/// A=00, C=01, G=11, T=10
pub fn encode(seq: &str) -> u64 {
    let mut encoded: u64 = 0;

    for char in seq.chars() {
        encoded = (encoded << 2) | ((char as u64 >> 1) & 3);
    }

    encoded
}

/// Decode u64s to up to a 32-ner using 2-bit encoding:
/// A=00, C=01, G=11, T=10
pub fn decode(kmer: &u64, k: u8) -> String {
    let mut kmer_mut = *kmer;
    let mut char_v: Vec<char> = vec![];

    for _ in 0..k {
        let bits = kmer_mut & 3;

        let c: char = match bits {
            0 => 'A',
            1 => 'C',
            3 => 'G',
            2 => 'T',
            _ => panic!("Invalid encoding value"),
        };

        char_v.push(c);
        kmer_mut >>= 2;
    }

    char_v.iter().rev().collect()
}