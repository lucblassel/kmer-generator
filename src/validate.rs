/// Check if user-supplied percentage value is valid
pub fn percent(s: &str) -> Result<f32, String> {
    let p: f32 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid f32 number", s))?;

    if p <= 1. && p >= 0. {
        Ok(p)
    } else {
        Err(format!("`{}` is not within the [0-1] range", p))
    }
}

/// Check if user-supplied query-number value is valid
pub fn query_number(s: &str) -> Result<u32, String> {
    let p: u32 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid u32 number", s))?;

    if p > 0 && p <= 1_000_000_000 {
        Ok(p)
    } else {
        Err(format!("`{}` is not within the [1-1e9] range", p))
    }
}

/// Check if user-supplied query-number value is valid
pub fn kmer_size(s: &str) -> Result<u8, String> {
    let p: u8 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid u8 number", s))?;

    if p > 0 && p <= 32 {
        Ok(p)
    } else {
        Err(format!("`{}` is not within the [1-32] range", p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_percent() -> Result<(), String> {
        let cases: [(&str, f32); 4] = [
            ("0", 0.),
            ("0.00001", 0.00001),
            ("0.9999999999999", 0.9999999999999),
            (".56", 0.56),
        ];
        for case in cases {
            assert_eq!(percent(case.0)?, case.1);
        }

        Ok(())
    }

    #[test]
    fn test_valid_number() -> Result<(), String> {
        let cases: [(&str, u32); 3] = [("10", 10), ("1", 1), ("1000000000", 1_000_000_000)];
        for case in cases {
            assert_eq!(query_number(case.0)?, case.1);
        }

        Ok(())
    }

    #[test]
    fn test_invalid_percent() {
        let cases: [&str; 3] = ["-0.1", "12", "dsds"];
        for case in cases {
            assert!(percent(case).is_err());
        }
    }

    #[test]
    fn test_invalid_number() {
        let cases: [&str; 4] = ["112.54", "10000000000", "dsds", "-2000"];
        for case in cases {
            assert!(query_number(case).is_err());
        }
    }

    #[test]
    fn test_invalid_size() {
        let cases: [&str; 4] = ["-20", "70", "dsds", "2.4"];
        for case in cases {
            assert!(kmer_size(case).is_err());
        }
    }
}
