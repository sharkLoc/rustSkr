use std::{
    fs::File,
    io::{self,BufRead,BufReader,Write},
};

#[derive(Debug)]
struct Df{
    a:u32,
    t:u32,
    g:u32,
    c:u32,
    n:u32,
    base:u32,
    read:u32,
    q20:u32,
    q30:u32,
}

impl Df{
    fn new() -> Self {
        Df { a: 0, t: 0, g: 0, c: 0, n: 0, base: 0, read: 0, q20: 0, q30: 0 }
    }
}

#[inline]
fn get_qval_num(val: &str, q:u8) -> u32 {
    val.as_bytes().iter().filter(|x| **x - 33 >= q ).collect::<Vec<&u8>>().len() as u32 
}   

#[inline]
fn get_bytes_count(src: &str, dest: u8) -> u32 {
    src.as_bytes().iter().filter(|x| **x == dest).collect::<Vec<&u8>>().len() as u32
}

fn out_file(name: String) -> Result<File, io::Error> {
    let fo = File::create(name)?;
    Ok(fo)
}

pub fn read_fq(name: String, out:String) -> Result<(), io::Error> {
	let fp = File::open(name)?;
    let mut fo = out_file(out)?;
    let reader = BufReader::new(fp);
    let mut n = 0;
    let mut count = Df::new();

    for line in reader.lines(){
        n += 1;
        if n == 1 || n == 3 {
            continue;
        }
        else if n == 2 {
            if let Ok(seq) = line {
                count.a += get_bytes_count(&seq,b'A');
                count.t += get_bytes_count(&seq,b'T');
                count.g += get_bytes_count(&seq,b'G');
                count.c += get_bytes_count(&seq,b'C');
                count.n += get_bytes_count(&seq,b'N');
                count.read += 1;
                count.base += seq.len() as u32; //count.a +count.t +count.g +count.c +count.n;
            }
        }
        else{
            n = 0;
            if let Ok(qval) = line {
                count.q20 += get_qval_num(&qval, 20);
                count.q30 += get_qval_num(&qval, 30);
            }

        }

    }

    fo.write(
        format!(
            "read average length:\t{:.0}\n\
             read GC content(%):\t{:.2}\n\
             total read Count:\t{}\n\
             total base Count:\t{}\n\n\
             base A Count:\t{}({:.2}%)\n\
             base T Count:\t{}({:.2}%)\n\
             base G Count:\t{}({:.2}%)\n\
             base C Count:\t{}({:.2}%)\n\
             base N Count:\t{}({:.2}%)\n\n\
             Number of base calls with quality value of 20 or higher (Q20+)\t{}({:.2}%)\n\
             Number of base calls with quality value of 30 or higher (Q30+)\t{}({:.2}%)\n",
            count.base as f64 / count.read as f64,
            (count.g + count.c)as f64 / count.base as f64 * 100f64,
            count.read, count.base, 
            count.a, count.a as f64 / count.base as f64 * 100f64, 
            count.t, count.t as f64 / count.base as f64 * 100f64,
            count.g, count.g as f64 / count.base as f64 * 100f64,
            count.c, count.c as f64 / count.base as f64 * 100f64,
            count.n, count.n as f64 / count.base as f64 * 100f64,
            count.q20, count.q20 as f64 / count.base as f64 * 100f64, 
            count.q30, count.q30 as f64 / count.base as f64 * 100f64,
        )
        .as_bytes()
    )?;
    Ok(())
}
