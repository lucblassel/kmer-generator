use clap::{Parser, Subcommand};

pub mod kmer;
pub mod sample;
pub mod validate;
pub mod commands;

#[derive(Parser, Debug)]
pub struct Args {
    /// Output file to write query k-mers to
    #[arg(short, long, global=true)]
    out: Option<String>,

    /// Size of k-mers
    #[arg(short,
        long = "kmer-size",
        default_value_t = 30,
        global=true,
        value_parser=validate::kmer_size)]
    k: u8,

    /// Number of query k-mers to generate
    #[arg(short,
        long = "query-number",
        default_value_t = 10_000,
        global=true,
        value_parser=validate::query_number)]
    n: usize,

    /// Output the query kmers in FASTA format
    #[arg(short = 'a', long, global=true)]
    fasta: bool,

    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate fully random kmers
    Generate,

    /// Mix real kmers with random kmers
    Mix {
        /// Percentage of real k-mers to generate when using a reference k-mer set of sequence
        #[arg(short,
            long,
            default_value_t=0.5,
            value_parser=validate::percent)]
        percent: f32,

        /// File containing k-mers to draw from (one per line)
        #[arg(short, long)]
        file: String,

        /// Shuffle the generated and sampled kemrs. 
        /// (If dealing with a large number of kmers set to false and use `shuf` on the output file)
        #[arg(short, long, default_value_t=false)]
        shuffle: bool,

        /// Return error if sample file does not contain enough kmers to reach specified percentage
        #[arg(short='t', long, default_value_t=false)]
        strict: bool
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args = Args::parse();

    match &args.command {
        Commands::Generate => {
            commands::full_random(args.out, args.n, args.k, args.fasta)?;
        },
        Commands::Mix {percent, file, shuffle, strict} => {
            let path: String = String::from(file);
            commands::mix(args.out, path, *percent, args.n, args.k, args.fasta, *shuffle, *strict)?;
        },
    }

    Ok(())

}
