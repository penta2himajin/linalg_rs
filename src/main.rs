extern crate linalg_rs;

use linalg_rs::{
    Scalar,
    Vector
};

fn main() {
    let s = Scalar::from(1);
    println!("{}", s.unwrap());
    let v = Vector::from(vec![1, 2]);
    println!("{:?}", v.unwrap());
}
