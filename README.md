# rustSkr
skrTools rust version.


## install
>install rust first
```
git clone https://github.com/sharkLoc/rustSkr.git && cd rustSkr
cargo b --release
```

```
rustSkr
skrTools rust version  ^_^

USAGE:
    rustSkr <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    fq2fa     trans fastq to fasta
    fastat    summary fasta file
    fqstat    summary fastq file
    filter    filter pe fastq files
    help      Print this message or the help of the given subcommand(s)
```

## filter fastq file
```
rustSkr filter  -h
rustSkr-filter
filter pe fastq files

USAGE:
    rustSkr filter [OPTIONS] --read1 <READ1> --read2 <READ2> --out1 <OUT1> --out2 <OUT2>

OPTIONS:
        --read1 <READ1>          input fq1 file name
        --read2 <READ2>          input fq2 file name
        --len <LEN>              reads length filter threshold [default: 0]
        --low <LOW>              base low quality threshold [default: 15]
        --low-rate <LOW_RATE>    reads low quality rate [default: 0.5]
        --mean <MEAN>            filter reads with low average quality less than [default: 30]
        --rate-n <RATE_N>        filter reads with N rate >= threshold [default: 0.05]
        --out1 <OUT1>            output clean fq1 file name
        --out2 <OUT2>            output clean fq2 file name
    -h, --help                   Print help information
```
