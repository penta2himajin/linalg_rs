use std::{
    clone::Clone,
    cmp::{PartialEq, PartialOrd},
    convert::From,
    marker::Copy,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    vec::Vec,
};

/*  Types   */
pub struct Scalar<
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
> {
    pub value: T,
}

#[derive(PartialEq)]
pub enum Direction {
    Row,
    Column,
}

pub struct Vector<
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Clone,
> {
    pub value: Vec<T>,
    pub direction: Direction,
}

pub struct Matrix<
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Clone,
> {
    pub value: Vec<Vec<T>>,
}

/*  Implementation  */
//Scalar
impl<T> Scalar<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    pub fn unwrap(self) -> T {
        self.value
    }
}

impl<T> From<T> for Scalar<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    fn from(s: T) -> Scalar<T> {
        Scalar { value: s }
    }
}

impl<T> Copy for Scalar<T> where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy
{
}

impl<T> Clone for Scalar<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    fn clone(&self) -> Scalar<T> {
        Scalar {
            value: self.value.clone(),
        }
    }
}

// Direction
impl Copy for Direction {}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        match self {
            Direction::Row => Direction::Row,
            Direction::Column => Direction::Column,
        }
    }
}

// Vector
impl<T> Vector<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Clone,
{
    pub fn unwrap(&self) -> Vec<T> {
        self.value.clone()
    }
}

impl<T> From<Vec<T>> for Vector<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Clone,
{
    fn from(v: Vec<T>) -> Vector<T> {
        Vector {
            value: v,
            direction: Direction::Column,
        }
    }
}

impl<T> Clone for Vector<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Clone,
{
    fn clone(&self) -> Vector<T> {
        Vector {
            value: self.value.clone(),
            direction: self.direction.clone(),
        }
    }
}

// Scalar and Scalar
impl<T> Add<Scalar<T>> for Scalar<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    type Output = Scalar<T>;

    fn add(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar {
            value: self.value + rhs.value,
        }
    }
}

impl<T> Sub<Scalar<T>> for Scalar<T>
where
    T: Add
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    type Output = Scalar<T>;

    fn sub(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar {
            value: self.value - rhs.value,
        }
    }
}

impl<T> Mul<Scalar<T>> for Scalar<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    type Output = Scalar<T>;

    fn mul(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar {
            value: self.value * rhs.value,
        }
    }
}

impl<T> Div<Scalar<T>> for Scalar<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    type Output = Scalar<T>;

    fn div(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar {
            value: self.value / rhs.value,
        }
    }
}

// Scalar and Vector
impl<T> Mul<&Vector<T>> for Scalar<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Vector<T> {
        let mut ret = rhs.value.clone();
        for i in 0..rhs.value.len() {
            ret[i] = self.value * rhs.value[i];
        }
        Vector::from(ret)
    }
}

impl<T> Mul<Scalar<T>> for &Vector<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    type Output = Vector<T>;

    fn mul(self, rhs: Scalar<T>) -> Vector<T> {
        let mut ret = self.value.clone();
        for i in 0..self.value.len() {
            ret[i] = rhs.value * self.value[i];
        }

        Vector::from(ret)
    }
}

// Vector and Vector
impl<T> Add<&Vector<T>> for &Vector<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Clone,
{
    type Output = Vector<T>;

    fn add(self, rhs: &Vector<T>) -> Vector<T> {
        if (self.value.len() == rhs.value.len()) && (self.direction == rhs.direction) {
            let mut ret = self.value.clone();

            for i in 0..self.value.len() {
                ret[i] += rhs.value[i].clone();
            }

            Vector {
                value: ret,
                direction: self.direction.clone(),
            }
        } else {
            panic!("length of vector are different.");
        }
    }
}

impl<T> Sub<&Vector<T>> for &Vector<T>
where
    T: Add
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Clone,
{
    type Output = Vector<T>;

    fn sub(self, rhs: &Vector<T>) -> Vector<T> {
        if (self.value.len() == rhs.value.len()) && (self.direction == rhs.direction) {
            let mut ret = self.value.clone();

            for i in 0..self.value.len() {
                ret[i] -= rhs.value[i].clone();
            }

            Vector {
                value: ret,
                direction: self.direction.clone(),
            }
        } else {
            panic!("length of vector are different.");
        }
    }
}

impl<T> Mul<&Vector<T>> for &Vector<T>
where
    T: Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div
        + DivAssign
        + PartialEq
        + PartialOrd
        + Clone,
{
    type Output = T;

    fn mul(self, rhs: &Vector<T>) -> T {
        if (self.value.len() == rhs.value.len()) && (self.direction == rhs.direction) {
            let mut v = self.value.clone();
            for i in 0..self.value.len() {
                v[i] *= rhs.value[i].clone();
            }

            let mut ret = v[0].clone();
            for i in 1..self.value.len() {
                ret += v[i].clone();
            }

            ret
        } else {
            panic!("length of vector are different.");
        }
    }
}
