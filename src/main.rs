use clap::Parser;
use rand::Rng;
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
    #[arg(short, long, default_value_t=String::from("stdout"))]
    out: String,

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

    println!("Args: {:?}", args);

    for _ in 1..5 {
        println!("{}", kmer::generate(args.k));
    }
}
