use rand::seq::SliceRandom;
use std::error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread;

use super::kmer;
use super::sample;

fn init_writer(out: Option<String>) -> Result<BufWriter<Box<dyn Write>>, std::io::Error> {
    let file: File;
    match out {
        Some(path) => {
            file = File::create(path)?;
            Ok(BufWriter::new(Box::new(file)))
        }
        None => Ok(BufWriter::new(Box::new(std::io::stdout()))),
    }
}

/// Output a list of fully random kmers
pub fn full_random(
    out: Option<String>,
    n: usize,
    k: u8,
    fasta: bool,
) -> Result<(), std::io::Error> {
    let mut writer = init_writer(out)?;

    if fasta {
        for i in 0..n {
            writeln!(writer, ">S{}", i)?;
            writeln!(writer, "{}", kmer::generate(k))?;
        }
    } else {
        for _ in 0..n {
            writeln!(writer, "{}", kmer::generate(k))?;
        }
    }

    writer.flush()?;

    Ok(())
}

/// Generate a vector of random kmers
fn generate_random(n: usize, k: u8) -> Vec<String> {
    let mut v: Vec<String> = Vec::with_capacity(n);

    for _ in 0..n {
        v.push(kmer::generate(k))
    }

    v
}

/// Generate random kmers and write them to a buffer
fn write_and_generate(
    k: u8,
    n: usize,
    writer: &mut BufWriter<Box<dyn Write>>,
    fasta: bool,
) -> Result<(), std::io::Error> {
    if fasta {
        for i in 0..n {
            writeln!(writer, ">S{}", i)?;
            writeln!(writer, "{}", kmer::generate(k))?;
        }
    } else {
        for _ in 0..n {
            writeln!(writer, "{}", kmer::generate(k))?;
        }
    }

    Ok(())
}

/// Write a list of kmers to a buffer
fn write_from_vec(
    kmers: Vec<String>,
    writer: &mut BufWriter<Box<dyn Write>>,
    fasta: bool,
) -> Result<(), std::io::Error> {
    if fasta {
        for (i, kmer) in kmers.iter().enumerate() {
            writeln!(writer, ">S{}", i)?;
            writeln!(writer, "{}", kmer)?;
        }
    } else {
        for kmer in kmers {
            writeln!(writer, "{}", kmer)?;
        }
    }
    Ok(())
}

/// Output a list with x% of real kmers sampled randomlly from a file and the rest of random kmers
pub fn mix(
    out: Option<String>,
    file: String,
    percent: f32,
    n: usize,
    k: u8,
    fasta: bool,
    shuffle: bool,
    strict: bool,
) -> Result<(), Box<dyn error::Error>> {
    let mut writer = init_writer(out)?;

    let n_real: usize = (n as f32 * percent).floor() as usize;
    let n_to_generate: usize = n - n_real;

    if shuffle {
        let sample_handle = thread::spawn(move || sample::lines(file, n_real, strict));
        let mut generated = generate_random(n_to_generate, k);

        let mut samples = sample_handle.join().unwrap()?;
        samples.append(&mut generated);
        samples.shuffle(&mut rand::thread_rng());

        write_from_vec(samples, &mut writer, fasta)?;
    } else {
        let sample_handle = thread::spawn(move || sample::lines(file, n_real, strict));
        write_and_generate(k, n_to_generate, &mut writer, fasta)?;
        let samples = sample_handle.join().unwrap()?;
        write_from_vec(samples, &mut writer, fasta)?;
    }

    Ok(())
}
