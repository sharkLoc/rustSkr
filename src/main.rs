mod fastq;
mod fasta;

use fastq::{
    fqtwofa::fq2fa,
    fqinfo::read_fq
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
            /// input fasta file name
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
    }
    
}
