use std::fs::File;
use std::io::{self,BufRead,BufReader};

use clap::Parser;

#[derive(Parser,Debug)]
struct Opt{
        /// input fastq file
        #[clap(short, long)]
        input: String,
}


fn main() {
        let cli = Opt::parse();
        let _ = fq2fa(cli.input);
}


fn fq2fa(name: String) -> Result<(), io::Error> {
        
		let fp = File::open(name)?;
        let reader = BufReader::new(fp);
        let mut num = 0;
        
		for line in reader.lines() {
            num += 1;
            if num == 1 || num == 2{
                if let Ok(line) = line {
                    if num == 1 {
                        println!("{}",line.replace("@", ">"));
                    }
                    else{
                        println!("{}",line);
                    }                    
                }
            }
            else if num == 4{
                num = 0;
            }
            else{
                continue;
            }
        }
        Ok(())
}
