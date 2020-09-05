extern crate linalg_rs;
use linalg_rs::{ Scalar, Vector };

fn main() {
    let s = Scalar::from(2);
    println!("{}", s.unwrap());
    let v_1 = Vector::from(vec![1, 2]);
    println!("{:?}", v_1.unwrap());
    let s_mul_v1 = s * &v_1;
    println!("{:?}", s_mul_v1.unwrap());
    let v_2 = Vector::from(vec![3, 4]);
    println!("{:?}", v_2.unwrap());
    let v1_add_v2 = &v_1 + &v_2;
    println!("{:?}", v1_add_v2.unwrap());
    let v1_sub_v2 = &v_1 - &v_2;
    println!("{:?}", v1_sub_v2.unwrap());
    let v1_v2_scoler_product = &v_1 * &v_2;
    println!("{}", v1_v2_scoler_product);
}