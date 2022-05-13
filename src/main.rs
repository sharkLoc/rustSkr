use std::fs::File;
use std::io::{self,BufRead,BufReader,Write};

use clap::Parser;

#[derive(Parser,Debug)]
struct Opt{
        /// input fastq file
        #[clap(short, long)]
        input: String,

		/// output file name
		#[clap(short, long, default_value_t = String::from("result.txt"))]
		output: String,
}


fn main() {
        let cli = Opt::parse();
        let _ = fq2fa(cli.input, cli.output);
}


fn fq2fa(name: String, out: String) -> Result<(), io::Error> {
        
		let fp = File::open(name)?;
		let mut fo = out_file(out)?;
        let reader = BufReader::new(fp);
        let mut num = 0;
        
		for line in reader.lines() {
            num += 1;
            if num == 1 || num == 2{
                if let Ok(line) = line {
                    if num == 1 {
						fo.write(
							format!(
								"{}\n", line.replace("@", ">")
							).as_bytes()
						).expect("output error!");
                    }
                    else{
						fo.write(
							format!(
								"{}\n",line
							).as_bytes()
						).expect("output error!");
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

fn out_file(out: String) -> Result<File, io::Error> {
	let fo = File::create(out)?;
	Ok(fo)
}
