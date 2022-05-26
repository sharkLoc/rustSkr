mod fastq;
mod fasta;

use fastq::{
    fqtwofa::fq2fa,
    fqinfo::read_fq,
    filter::read_pairs
};
use fasta::fainfo::read_fa;

use clap::{Parser, Subcommand};

///skrTools rust version  ^_^
#[derive(Parser,Debug)]
struct Opt{
        #[clap(subcommand)]
        cli: Sub,
}

#[derive(Subcommand,Debug)]
enum Sub{
        /// trans fastq to fasta
        Fq2fa{
                /// input fastq file name
                #[clap(short, long)]
                input: String,
                /// output fasta file name
                #[clap(short, long)]
                output: String,
        },
        /// summary fasta file
        Fastat{
            /// input fasta[.gz] file name
            #[clap(short, long)]
            input: String,
            /// output fasta file name
            #[clap(short, long, default_value_t = String::from("summary.txt"))]
            output: String,
        },
        /// summary fastq file
        Fqstat{
                /// input fastq file name
                #[clap(short, long)]
                input: String,
                /// output summary result
                #[clap(short,long)]
                output: String,
        },
        ///filter pe fastq files.
        Filter{
            ///input fq1 file name.
            #[clap(long)]
            read1: String,
            ///input fq2 file name.
            #[clap(long)]
            read2: String,
            /// reads length filter threshold.
            #[clap(long, default_value_t=0)]
            len: u32,
            ///base low quality threshold.
            #[clap(long, default_value_t=15)]
            low: u8,
            ///reads low quality rate.
            #[clap(long, default_value_t=0.50)]
            low_rate: f64,
            ///filter reads with low average quality less than
            #[clap(long, default_value_t=30.0)]
            mean: f64,
            ///filter reads with N rate >= threshold.
            #[clap(long, default_value_t=0.05)]
            rate_n: f64,
            ///output clean fq1 file name
            #[clap(long)]
            out1: String,
            ///output clean fq2 file name
            #[clap(long)]
            out2: String,
        }
}

fn main() {
    let parse = Opt::parse();
    
    match parse.cli {
        Sub::Fq2fa { input, output } => {
            let _x = fq2fa(input, output);
        },
        Sub::Fastat { input, output } => {
            let _x = read_fa(input, output);
        }
        Sub::Fqstat { input, output } => {
            let _x = read_fq(input,output);
        },
        Sub::Filter { read1, read2, len, low, low_rate, mean, rate_n, out1, out2 } => {
            let _x = read_pairs(read1, read2, len, low, low_rate, mean, rate_n, out1, out2);
        }
    }
    
}
