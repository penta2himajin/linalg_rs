extern crate linalg_rs;
use linalg_rs::{ Scalar, Vector };

fn main() {
    let s = Scalar::from(2);
    println!("Scalar (2)   : {}", s.unwrap());
    let v_1 = Vector::from(vec![1, 2]);
    println!("Vector [1, 2]: {:?}", v_1.unwrap());
    let s_mul_v1 = s * &v_1;
    println!("(2) * [3, 4] : {:?}", s_mul_v1.unwrap());
    let v_2 = Vector::from(vec![3, 4]);
    println!("Vector [3, 4]: {:?}", v_2.unwrap());
    let v1_add_v2 = &v_1 + &v_2;
    println!("[1,2] + [3,4]: {:?}", v1_add_v2.unwrap());
    let v1_sub_v2 = &v_1 - &v_2;
    println!("[1,2] - [3,4]: {:?}", v1_sub_v2.unwrap());
    let v1_v2_scoler_product = &v_1 * &v_2;
    println!("[1,2] * [3,4]: {}", v1_v2_scoler_product);
}