use std::fs::File;
use std::io::{self,Read,Write};
use std::collections::HashMap;


#[derive(Debug)]
#[allow(non_snake_case)]
struct Datf {
    A: u32,
    T: u32,
    G: u32,
    C: u32,
    N: u32,
    GC: f64,
    LEN: u32,
}

impl Datf {
    fn new() -> Self {
        Datf { A: 0, T: 0, G: 0, C: 0, N: 0, GC: 0f64, LEN: 0 }
    }
}

pub fn read_fa(name: String, out: String) -> Result<(),io::Error> {
    let mut fp = File::open(name)?;
    let mut fo = out_file(out)?;
    
    let mut txt = String::new();
    let _tmp = fp.read_to_string(&mut txt)?;
    let mut content: Vec<&str> = txt.split(">").collect();
    let _tmp = content.remove(0); // skip first ">"

    let mut hash = HashMap::new();
    let mut chrs = Vec::new();
    let mut total = Datf::new();

    for black in content {
        let mut this_seq = Datf::new();
        let id_seq = black.split_once("\n").unwrap();
        chrs.push(id_seq.0);
        let mut seq = id_seq.1.to_uppercase();
        seq.retain(|c| c != '\n'); // skip each "\n"  in seq
        
        this_seq.A = seq.as_bytes().iter().filter(|x| **x == b'A').collect::<Vec<&u8>>().len() as u32;
        this_seq.T = seq.as_bytes().iter().filter(|x| **x == b'T').collect::<Vec<&u8>>().len() as u32;
        this_seq.G = seq.as_bytes().iter().filter(|x| **x == b'G').collect::<Vec<&u8>>().len() as u32;
        this_seq.C = seq.as_bytes().iter().filter(|x| **x == b'C').collect::<Vec<&u8>>().len() as u32;
        this_seq.N = seq.as_bytes().iter().filter(|x| **x == b'N').collect::<Vec<&u8>>().len() as u32;
        this_seq.LEN = this_seq.A + this_seq.T + this_seq.G + this_seq.C + this_seq.N;
        this_seq.GC = (this_seq.G + this_seq.C) as f64 / this_seq.LEN as f64;
        hash.insert(id_seq.0, this_seq);
    }

    for (_k,v) in &hash {
        total.A += v.A;
        total.T += v.T;
        total.G += v.G;
        total.C += v.C;
        total.N += v.N;
        total.LEN += v.LEN;
    }
    total.GC = (total.G + total.C) as f64 / total.LEN as f64;
    hash.insert("All", total);
    chrs.push("All");

    fo.write(
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            "chr_Name", "base_A", "base_T", "base_G", "base_C", "base_N", "GC_Rate", "seq_Len"
        )
        .as_bytes(),
    )?;
    
    for n in chrs {
        if let Some(df )= hash.get(n) {
            fo.write(
                format!(
                    "{}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:.2}%\t{:?}\n",
                    n, df.A, df.T, df.G, df.C, df.N, df.GC * 100f64, df.LEN
                )
                .as_bytes()
            )?;
        }
    }
    Ok(())
}


fn out_file(name: String) -> Result<File, io::Error> {
    let fo = File::create(name)?;
    Ok(fo)
}