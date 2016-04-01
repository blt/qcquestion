extern crate qcq;

fn main() {
    let rev = qcq::qcq::rev::reverse(&[1,2,3]);
    println!("{:?}", rev);

    let b = qcq::qcq::rev::parse(b"aaabccc");
    println!("{:?}", b);
}
