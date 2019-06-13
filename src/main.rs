extern crate linalg_rs;

use linalg_rs::{
    Scalar,
    Vector
};

fn main() {
    let n = Scalar{ value: 1 };
    let s = Scalar::from(n);
    println!("{}", s.unwrap());
    let v = Vector::from(vec![1, 2, 3]);
}