extern crate easytiming;

use std::io;
use easytiming::Timing;

#[test]
fn println() {
    let t: Timing = Timing::new("simple");
    println!("{:?}", t);
    println!("{}", t);
    assert!(true);
}

#[test]
fn write() {
    let t: Timing<io::Sink> = Timing::with_writer("write", io::sink());
    println!("{:?}", t);
    println!("{}", t);
    assert!(true);
}


#[test]
fn catch() {
    let out = Vec::<u8>::new();
    {
        let _t: Timing<Vec<_>> = Timing::with_writer("catch", out);
    }
    // println!("{:?}", out);
    assert!(true);
}
