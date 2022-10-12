# kmer-generator

This is a command line tool to generate lists of kmers for benchmarking purposes. It can either generate fully random k-mers, or also sample kmers from an existing file with a certain real to generated ratio. 

```
Usage: kmer-generator [OPTIONS] <COMMAND>

Commands:
  generate  Generate fully random kmers
  mix       Mix real kmers with random kmers
  help      Print this message or the help of the given subcommand(s)

Options:
  -o, --out <OUT>         Output file to write query k-mers to
  -k, --kmer-size <K>     Size of k-mers [default: 30]
  -n, --query-number <N>  Number of query k-mers to generate [default: 10000]
  -a, --fasta             Output the query kmers in FASTA format
  -h, --help              Print help information
```

## `generate` subcommand

```
Generate fully random kmers

Usage: kmer-generator generate [OPTIONS]

Options:
  -o, --out <OUT>         Output file to write query k-mers to
  -k, --kmer-size <K>     Size of k-mers [default: 30]
  -n, --query-number <N>  Number of query k-mers to generate [default: 10000]
  -a, --fasta             Output the query kmers in FASTA format
  -h, --help              Print help information
```

## `mix` subcommand

```
Mix real kmers with random kmers

Usage: kmer-generator mix [OPTIONS] --file <FILE>

Options:
  -o, --out <OUT>          Output file to write query k-mers to
  -p, --percent <PERCENT>  Percentage of real k-mers to generate when using a reference k-mer set of sequence [default: 0.5]
  -f, --file <FILE>        File containing k-mers to draw from (one per line)
  -k, --kmer-size <K>      Size of k-mers [default: 30]
  -n, --query-number <N>   Number of query k-mers to generate [default: 10000]
  -s, --shuffle            Shuffle the generated and sampled kemrs. (If dealing with a large number of kmers set to false and use `shuf` on the output file)
  -a, --fasta              Output the query kmers in FASTA format
  -t, --strict             Return error if sample file does not contain enough kmers to reach specified percentage
  -h, --help               Print help information
```