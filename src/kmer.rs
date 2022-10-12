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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_encode() {
        let cases: [(&str, u64); 9] = [
            (
                "AACGATATGTAGGGGAATAAG",
                0b000001110010001011100011111111000010000011,
            ),
            (
                "CGGGAGGCGGGGTGCTCAATA",
                0b011111110011110111111111101101100100001000,
            ),
            (
                "ACTGATTAGCAACGAGTGGGG",
                0b000110110010100011010000011100111011111111,
            ),
            (
                "CACATGACGATCAGTAGAACA",
                0b010001001011000111001001001110001100000100,
            ),
            (
                "CGCAGTCTCTCAGTCTCCCAC",
                0b011101001110011001100100111001100101010001,
            ),
            (
                "GAAGCCTAGCGGGCGTTCGGC",
                0b110000110101100011011111110111101001111101,
            ),
            (
                "TTCATGTCTCAGACCTGTGCA",
                0b101001001011100110010011000101101110110100,
            ),
            (
                "TCCTACGTAGCAGGTCACGAA",
                0b100101100001111000110100111110010001110000,
            ),
            (
                "CAATTAGAGTCGAGCTTCGAG",
                0b010000101000110011100111001101101001110011,
            ),
        ];
        for case in cases {
            assert_eq!(case.1, encode(case.0));
        }
    }

    #[test]
    fn encode_decode() {
        let cases: [&str; 9] = [
            "AACGATATGTAGGGGAATAAG",
            "CGGGAGGCGGGGTGCTCAATA",
            "ACTGATTAGCAACGAGTGGGG",
            "CACATGACGATCAGTAGAACA",
            "CGCAGTCTCTCAGTCTCCCAC",
            "GAAGCCTAGCGGGCGTTCGGC",
            "TTCATGTCTCAGACCTGTGCA",
            "TCCTACGTAGCAGGTCACGAA",
            "CAATTAGAGTCGAGCTTCGAG",
        ];
        for case in cases {
            assert_eq!(case, decode(&encode(case), 21));
        }
    }
}
