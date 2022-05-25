use std::{
    fs::File,
    io::{self,BufRead,BufReader,Write}
};
use crate::fasta::fainfo::out_file;

struct Record{
    id: String,
    seq: String,
    qual: Vec<u8>,
}

struct Log{
    raw_reads_num: u32,
    raw_base_num: u32,
    clean_reads_num: u32,
    clean_base_num: u32,
}

impl Record {
    fn new() -> Self {
        Record { id: String::new(), seq: String::new(), qual: vec![] }
    }

    fn get_len(&self) -> u32 {
        self.seq.len() as u32
    }
    
    fn get_average_qual(&self, len: u32) -> f64 {
        let mut sum = 0u32;
        for i in self.qual.iter() {
            sum += *i as u32; 
        }
        sum as f64 / len as f64 - 33.0
    }

    fn get_lowqual_rate(&self,low: u8, len: u32) -> f64 {
        self.qual.iter().filter(|x| **x - 33 < low).collect::<Vec<&u8>>().len() as f64 / len as f64
    }

    fn get_nrate(&self, len: u32) -> f64 {
        self.seq.as_bytes().iter().filter(|x| **x == b'N').collect::<Vec<&u8>>().len() as f64 / len as f64
    }

    fn record_out(&self) -> String {
         format!(
            //"{}\n{}\n+\n{:?}\n",self.id,self.seq,self.qual
            "{}\n{}\n+\n{}\n",self.id,self.seq,std::str::from_utf8(&self.qual).unwrap()
        )
    }

    fn clear(&mut self) {
        self.id.clear();
        self.seq.clear();
        self.qual.clear();
    }
}

impl Log{
    fn new() -> Self {
        Log { raw_reads_num: 0, raw_base_num: 0, clean_reads_num: 0, clean_base_num: 0 }
    }

    fn show(&self) {
        println!("total raw reads number in pairs fastq file:\t{}\ntotal raw base number in pairs fastq file:\t{}\ntotal clean reads number in pairs fastq file:\t{}\ntotal clean base number in pairs fastq file:\t{}",
            self.raw_reads_num,self.raw_base_num,self.clean_reads_num,self.clean_base_num
        );
    }
}

pub fn read_pairs(read1:String, read2: String,
                  len: u32 ,low:u8, qual:f64,
                  mean: f64, rate: f64, out1: String, out2: String) -> Result<(), io::Error> {

    let fp1 = File::open(&read1)?;
    let fp2 = File::open(&read2)?;
    let mut fo1 = out_file(out1)?;
    let mut fo2 = out_file(out2)?;
    let reader1 = BufReader::new(&fp1);
    let reader2 = BufReader::new(&fp2);

    let mut n = 0;
    let mut log = Log::new();
    let mut record1 = Record::new();
    let mut record2  = Record::new();

    for (seq1,seq2) in reader1.lines().zip(reader2.lines()) {
        n += 1;
        match n {
            1 => {
                if let Ok(seq) = seq1 { record1.id = seq; }
                if let Ok(seq) = seq2 { record2.id = seq; }
                continue;
            },
            2 => {
                if let Ok(seq) = seq1 { record1.seq = seq; }
                if let Ok(seq) = seq2 { record2.seq = seq; }
                continue;
            },
            3 => { continue; },
            4 => { 
                if let Ok(seq) = seq1 { record1.qual = seq.into_bytes(); }
                if let Ok(seq) = seq2 { record2.qual = seq.into_bytes(); }
                n = 0; 
            },
            _ => {unreachable!();}
        }

        let r1_len = record1.get_len();
        let r1_ave = record1.get_average_qual(r1_len);
        let r1_low = record1.get_lowqual_rate(low, r1_len);
        let r1_n = record1.get_nrate(r1_len);
        let r2_len = record2.get_len(); 
        let r2_ave = record2.get_average_qual(r2_len);
        let r2_low = record2.get_lowqual_rate(low, r2_len);
        let r2_n = record2.get_nrate(r2_len);

        log.raw_reads_num += 2;
        log.raw_base_num += r1_len + r2_len;

        if r1_len < len || r2_len < len { continue; }
        if r1_ave < mean || r2_ave < mean { continue; }
        if r1_low > qual || r2_low > qual { continue; }
        if r1_n >= rate || r2_n >= rate { continue; }

        log.clean_reads_num += 2;
        log.clean_base_num += r1_len + r2_len;

        fo1.write(record1.record_out().as_bytes())?;
        fo2.write(record2.record_out().as_bytes())?;
        
        record1.clear();
        record2.clear();
    }
    log.show();

    Ok(())
}