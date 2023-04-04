use compressor::compress;
use compressor::FinalResult;
use std::env;
fn main() {
    let result:FinalResult = compress(env::args()); 
    println!("Sorce len: {:?}",result.source_len);
    println!("Target len: {:?}",result.destnt_len);
    println!("Elapsed: {:?}",result.time_elapsed);
}