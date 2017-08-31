extern crate easytiming;

use easytiming::Timing;

#[test]
fn simple() {
    let t = Timing::new("simple");
    println!("{:?}", t);
    assert!(false);
}
