extern crate flate2;
use std::time::Duration;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

pub struct FinalResult{
    pub source_len: u64,
    pub destnt_len: u64,
    pub time_elapsed: Duration,
}

pub fn compress(mut args: env::Args) -> FinalResult{
    if args.len() != 3 {
        panic!("Problem parsing arguments:");
    }
    args.next();
    let mut input = match args.next(){
        Some(arg) => BufReader::new(File::open(arg).unwrap()),
        None => panic!("Error Reading the File"),
    };
    // let mut input = BufReader::new(File::open().unwrap());  //File reading
    let output = match args.next(){
        Some(arg) => File::create(arg).unwrap(),
        None => panic!("Error Creating the File"),
    };
    let mut encoder = GzEncoder::new(output,Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    let x = input.get_ref().metadata().unwrap().len();
    let y = output.metadata().unwrap().len();
    let time = start.elapsed();
    FinalResult{
        source_len:x,
        destnt_len:y,
        time_elapsed:time,
    }
}