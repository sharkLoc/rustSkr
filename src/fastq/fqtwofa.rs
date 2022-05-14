use std::fs::File;
use std::io::{self,BufRead,BufReader,Write};


pub fn fq2fa(name: String, out:String) -> Result<(),io::Error> {
    let fp = File::open(name)?;
    let mut fo = out_file(out)?;
    let reader = BufReader::new(fp);
    let mut num = 0;

    for line in reader.lines() {
        num += 1;
        if num == 1 || num == 2  {
            if let Ok(line) = line {
                if num == 1 {
                    fo.write(
                        format!(
                            "{}\n",line.replace("@", ">")
                        ).as_bytes()
                    )?;
                } 
                else {
                    fo.write(
                        format!(
                            "{}\n",line
                        ).as_bytes()
                    )?;
                }
            }
        }
        else if num == 4 {
            num = 0;
        }
        else { 
            continue;
        }
    }
    Ok(())
}

fn out_file(name: String) -> Result<File, io::Error> {
    let fo = File::create(name)?;
    Ok(fo)
}