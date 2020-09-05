use std::{
    vec::Vec,
    convert::From,
    clone::Clone,
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign,
        Mul,
        MulAssign,
        Div,
        DivAssign
    },
    cmp::{
        PartialEq,
        PartialOrd
    },
    marker::{
        Copy
    }
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
     + Copy> {
    pub value: T
}

pub enum Direction {
    Row, Column
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
     + Clone> {
    pub value: Vec<T>,
    pub direction: Direction
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
     + Clone> {
    pub value: Vec<T>,
}

/*  Implementation  */
//Scalar
impl<T> Scalar<T> 
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
    pub fn unwrap(self) -> T {
        self.value
    }
}

impl<T> From<T> for Scalar<T>
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
    fn from(s: T) -> Scalar<T> {
        Scalar { value: s }
    }
}

impl<T> Copy for Scalar<T>
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
}

impl<T> Clone for Scalar<T>
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
    fn clone(&self) -> Scalar<T> {
        Scalar{ value: self.value.clone() }
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
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Clone {
    pub fn unwrap(&self) -> Vec<T> {
        self.value.clone()
    }
}

impl<T> From<Vec<T>> for Vector<T>
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Clone {
    fn from(v: Vec<T>) -> Vector<T> {
        Vector {
	        value: v,
            direction: Direction::Column
	    }
    }
}

impl<T> Clone for Vector<T>
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Clone {
    fn clone(&self) -> Vector<T> {
        Vector {
            value: self.value.clone(),
            direction: self.direction.clone()
	    }
    }
}

// Scalar and Scalar
impl<T> Add<Scalar<T>> for Scalar<T>
where T: Add<Output=T>
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
    type Output = Scalar<T>;

    fn add(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar{ value: self.value + rhs.value }
    }
}

impl<T> Sub<Scalar<T>> for Scalar<T>
where T: Add
       + AddAssign
       + Sub<Output=T>
       + SubAssign
       + Mul
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
    type Output = Scalar<T>;

    fn sub(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar{ value: self.value - rhs.value }
    }
}

impl<T> Mul<Scalar<T>> for Scalar<T>
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul<Output=T>
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
    type Output = Scalar<T>;

    fn mul(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar{ value: self.value * rhs.value }
    }
}

impl<T> Div<Scalar<T>> for Scalar<T>
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul
       + MulAssign
       + Div<Output=T>
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
    type Output = Scalar<T>;

    fn div(self, rhs: Scalar<T>) -> Scalar<T> {
        Scalar{ value: self.value / rhs.value }
    }
}

// Scalar snd Vector
impl<T> Mul<Vector<T>> for Scalar<T>
where T: Add
       + AddAssign
       + Sub
       + SubAssign
       + Mul<Output=T>
       + MulAssign
       + Div
       + DivAssign
       + PartialEq
       + PartialOrd
       + Copy {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        let mut ret = rhs.value.clone();
        for i in 0..rhs.value.len() {
            ret[i] = self.value * rhs.value[i];
        }
        
        Vector::from(ret)
    }
}
