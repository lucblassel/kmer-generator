use amadeus_streaming::SampleUnstable;
use rand::{self};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

/// Count newline characters in a file
pub fn count(path: PathBuf) -> io::Result<usize> {
    const CAP: usize = 1024 * 128;

    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(CAP, file);
    let mut newlines: usize = 0;

    loop {
        let length = {
            let buffer = reader.fill_buf()?;
            newlines += bytecount::count(buffer, b'\n');
            buffer.len()
        };

        if length == 0 {
            break;
        }

        reader.consume(length);
    }

    Ok(newlines)
}

/// Sample n lines from a file of unknown length using reservoir sampling
pub fn lines(path: String, num: usize, strict:bool) -> Result<Vec<String>,String> {
    let mut sampler = SampleUnstable::new(num);

    let file: File = File::open(path).unwrap();
    let reader = BufReader::with_capacity(1024 * 128, file);

    for line in reader.lines() {
        let l = match line {
            Ok(l) => l,
            Err(e) => panic!("cannot read line! {}", e),
        };

        sampler.push(l, &mut rand::thread_rng());
    }

    let samples = sampler.into_iter().collect::<Vec<String>>();

    if samples.len() != num && strict{
        Err(format!("Reference file not big enough to select {} kmers.", num))
    } else {
        Ok(samples)
    }

}
