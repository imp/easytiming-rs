extern crate easytiming;

use easytiming::Timing;

#[test]
fn simple() {
    let t: Timing = Timing::new("simple");
    println!("{:?}", t);
    println!("{}", t);
    assert!(true);
}
