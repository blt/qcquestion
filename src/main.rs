extern crate qcq;

use std::str;

fn main() {
    let rev = qcq::qcq::rev::reverse(&[1,2,3]);
    println!("{:?}", rev);
}
