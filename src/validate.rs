/// Check if user-supplied percentage value is valid
pub fn percent(s: &str) -> Result<f32, String> {
    let p: f32 = s.parse().map_err(|_| format!("`{}` isn't a valid f32 number", s))?;

    if p <= 1. && p >= 0. {
        Ok(p)
    } else {
        Err(format!("`{}` is not within the [0-1] range", p))
    }
}

/// Check if user-supplied query-number value is valid
pub fn query_number(s: &str) -> Result<u32, String> {
    let p: u32 = s.parse().map_err(|_| format!("`{}` isn't a valid u32 number", s))?;

    if p > 0 && p <= 1_000_000_000 {
        Ok(p)
    } else {
        Err(format!("`{}` is not within the [1-1e9] range", p))
    }
}

/// Check if user-supplied query-number value is valid
pub fn kmer_size(s: &str) -> Result<u8, String> {
    let p: u8 = s.parse().map_err(|_| format!("`{}` isn't a valid u8 number", s))?;

    if p > 0 && p <= 32 {
        Ok(p)
    } else {
        Err(format!("`{}` is not within the [1-32] range", p))
    }
}