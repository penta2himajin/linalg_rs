extern crate num;

use std::{
    vec::Vec,
    clone::Clone,
    ops::{
        Add,
        Sub,
        Mul,
        Div
    }
};
use num::{
    Integer
};

/*  Types   */
pub struct Scalar<T: Integer> {
    value: T
}

pub enum Direction {
    Row, Column
}

pub struct Vector<T: Integer> {
    value: Vec<T>,
    direction: Direction
}
pub struct Matrix<T: Integer> {
    value: Vec<T>,
}

/*  Implementation  */
//Scalar
impl<T> Scalar<T> where T: Integer {
    fn from(s: T) -> Scalar<T> {
        Scalar{
            value: s
        }
    }

    pub fn unwrap(self) -> T {
        self.value
    }
}

// Vector
impl<T> Vector<T> where T: Integer {
    fn from(v: Vec<T>) -> Vector<T> {
        Vector{
                value: v,
                direction: Direction::Column
        }
    }

    fn unwrap(self) -> Vec<T> {
        self.value
    }
}

// Scalar and Scalar
impl<T> Add<Scalar<T>> for Scalar<T> where T: Integer {
    type Output = Scalar<T>;

    fn add(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar{ value: self.value + rhs.value }
    }
}

impl<T> Sub<Scalar<T>> for Scalar<T> where T: Integer {
    type Output = Scalar<T>;

    fn sub(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar{ value: self.value - rhs.value }
    }
}

impl<T> Mul<Scalar<T>> for Scalar<T> where T: Integer {
    type Output = Scalar<T>;

    fn mul(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar{ value: self.value * rhs.value }
    }
}

impl<T> Div<Scalar<T>> for Scalar<T> where T: Integer {
    type Output = Scalar<T>;

    fn div(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar{ value: self.value / rhs.value }
    }
}

// Scalar snd Vector
impl<T> Mul<Vector<T>> for Scalar<T> where T: Integer {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        let mut ret = rhs.value.clone();
        for i in 0..rhs.value.len() {
            ret[i] = self.value * rhs.value[i];
        }
        
        Vector::from(ret)
    }
}
