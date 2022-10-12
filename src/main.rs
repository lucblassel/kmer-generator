use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub mod kmer;
pub mod validate;

#[derive(Parser, Debug)]
pub struct Args {
    /// File containing k-mers to draw from (one per line)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Fasta file containing a reference sequence to generate sequential k-mers from
    #[arg(short, long)]
    reference: Option<PathBuf>,

    /// Output file to write query k-mers to
    #[arg(short, long)]
    out: Option<String>,

    /// Size of k-mers
    #[arg(short,
        long = "kmer-size",
        default_value_t = 30,
        value_parser=validate::kmer_size)]
    k: u8,

    /// Number of query k-mers to generate
    #[arg(short,
        long = "query-number",
        default_value_t = 1_000_000,
        value_parser=validate::query_number)]
    n: u32,

    /// Percentage of random k-mers to generate when using a reference k-mer set of sequence
    #[arg(short,
        long,
        default_value_t=0.5,
        value_parser=validate::percent)]
    percent: f32,

    /// Output the query kmers in FASTA format
    #[arg(short = 'a', long)]
    fasta: bool,
}

fn main() {
    let args = Args::parse();
    let file: File;

    let mut writer: BufWriter<Box<dyn Write>> = match args.out {
        Some(path) => {
            file = File::create(path).expect("Error creating file");
            BufWriter::new(Box::new(file))
        }
        None => BufWriter::new(Box::new(std::io::stdout())),
    };

    for _ in 0..args.n {
        writeln!(writer, "{}", kmer::generate(args.k)).expect("Problem writing kmer");
    }

    writer.flush().expect("Problem flushing writer");
}
