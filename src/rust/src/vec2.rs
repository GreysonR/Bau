// Modified version of [vec2](https://crates.io/crates/vec2) that removes non standard stuff

use std::ops::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use crate::Geo;

/// Generic vector with two components.
///
/// It implements multiple operators (for each combination of owned and borrowed
/// args), namely addition, subtraction, element-wise multiplication,
/// element-wise division and multiplication & division by a number. (Note that
/// you can only multiply and divide in the following order: `vector op number`,
/// since it is not possible to implement a foreign trait on `T`.)
///
/// This crate exports a specific version of [`Vec2`](crate::vec2::Vec2) with
/// [`f64`](f64) components — [`Fecc`](crate::fecc::Fecc). It implements
/// additional methods and is heavily inspired by [`p5.Vector`](https://p5js.org/reference/#/p5.Vector).
///
/// # Examples
///
/// Basic arithmetic.
///
/// ```
/// use vec2::Vec2;
///
/// let a = Vec2::new(3_i32, 4);
/// let b = a * 5; // (15, 20)
/// let c = Vec2::new(-10, -8);
/// let d = b - c; // (5, 12)
/// let e = -d; // (-5, -12)
/// ```
///
/// Shorthand construction using [`From`](std::convert::From).
///
/// ```
/// use vec2::Vec2;
///
/// let a: Vec2<i32> = (10, 5).into();
/// ```
///
/// Using [`Fecc`](crate::fecc::Fecc)'s extended API.
///
/// ```
/// # use float_cmp::assert_approx_eq;
/// # use std::f64::consts::PI;
/// use vec2::Fecc;
///
/// let a: Fecc = (3.0, 4.0).into();
/// let b = a / 0.2; // (15.0, 20.0)
/// let c = b.limit(20.0); // (12.0, 16.0)
/// let d = c.rotate(PI); // (-12.0, -16.0)
/// let e = d.turn(0.0); // (20.0, 0.0)
///
/// assert_approx_eq!(f64, e.mag(), 20.0);
/// ```
#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Default, Debug, Serialize,  Deserialize)]
pub struct Vec2 {
    #[allow(missing_docs)]
    pub x: Geo,

    #[allow(missing_docs)]
    pub y: Geo,
}


#[wasm_bindgen]
impl Vec2 {
    /// Constructs a new vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec2::Vec2;
    ///
    /// let a: Vec2<i32> = Vec2::new(10, 0);
    /// ```
    ///
    /// You can also construct it from a tuple:
    ///
    /// ```
    /// use vec2::Vec2;
    ///
    /// let a: Vec2<i32> = (10, 0).into();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(x: Geo, y: Geo) -> Self {
        Self { x, y }
    }

    /// Takes a dot product of the vector with another.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec2::Vec2;
    ///
    /// let a: Vec2<i32> = Vec2::new(10, 0);
    /// let b: Vec2<i32> = Vec2::new(5, 0);
    ///
    /// assert_eq!(a.dot(b), 50);
    /// ```
    pub fn dot(self, rhs: Vec2) -> Geo {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Takes the cross-product (a scalar) of the vector with another.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec2::Vec2;
    ///
    /// let a: Vec2<i32> = Vec2::new(10, 0);
    /// let b: Vec2<i32> = Vec2::new(0, -10);
    ///
    /// assert_eq!(a.cross(b), -100);
    /// ```
    pub fn cross(self, rhs: Vec2) -> Geo {
        self.x * rhs.y - self.y * rhs.x
    }
    
    /// Performs element-wise [`min`](std::cmp::Ord::min).
    ///
    /// # Examples
    ///
    /// ```
    /// use vec2::Vec2;
    ///
    /// let a: Vec2<i32> = Vec2::new(-100, 100);
    /// let b: Vec2<i32> = Vec2::new(0, 0);
    /// let min = a.min(b);
    ///
    /// assert_eq!(min.x, -100);
    /// assert_eq!(min.y, 0);
    /// ```
    pub fn min(self, rhs: Vec2) -> Vec2 {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    /// Performs element-wise [`max`](std::cmp::Ord::max).
    ///
    /// # Examples
    ///
    /// ```
    /// use vec2::Vec2;
    ///
    /// let a: Vec2<i32> = Vec2::new(-100, 100);
    /// let b: Vec2<i32> = Vec2::new(0, 0);
    /// let max = a.max(b);
    ///
    /// assert_eq!(max.x, 0);
    /// assert_eq!(max.y, 100);
    /// ```
    pub fn max(self, rhs: Vec2) -> Vec2 {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Performs element-wise [`clamp`](std::cmp::Ord::clamp).
    ///
    /// # Examples
    ///
    /// ```
    /// use vec2::Vec2;
    ///
    /// let a: Vec2<i32> = Vec2::new(-100, 100);
    /// let min: Vec2<i32> = Vec2::new(0, 10);
    /// let max: Vec2<i32> = Vec2::new(0, 10);
    /// let clamped = a.clamp(min, max);
    ///
    /// assert_eq!(clamped.x, 0);
    /// assert_eq!(clamped.y, 10);
    /// ```
    pub fn clamp(self, min: Vec2, max: Vec2) -> Vec2 {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}

impl From<(Geo, Geo)> for Vec2 {
    /// Constructs a new vector from a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec2::Vec2;
    ///
    /// let a: Vec2<i32> = (10, 0).into();
    /// ```
    fn from((x, y): (Geo, Geo)) -> Self {
        Self { x, y }
    }
}

#[allow(clippy::from_over_into)]
impl Into<(Geo, Geo)> for Vec2 {
    fn into(self) -> (Geo, Geo) {
        (self.x, self.y)
    }
}

// Unary operators.

// Neg.

// Owned.
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

// Borrowed.
impl Neg for &Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}


// Binary operators.

// Add.

// Owned & owned.
impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

// Owned & borrowed.
impl Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

// Borrowed & owned.
impl Add<Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

// Borrowed & borrowed.
impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

// Sub.

// Owned & owned.
impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

// Owned & borrowed.
impl Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

// Borrowed & owned.
impl Sub<Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

// Borrowed & borrowed.
impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

// Mul with T.

// Owned & owned.
impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

// Owned & borrowed.
impl Mul<&Geo> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &Geo) -> Self::Output {
        Vec2 {
            x: self.x.mul(*rhs),
            y: self.y.mul(*rhs),
        }
    }
}

// Borrowed & owned.
impl Mul for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

// Borrowed & borrowed.
impl Mul<&Geo> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &Geo) -> Self::Output {
        Vec2 {
            x: self.x.mul(*rhs),
            y: self.y.mul(*rhs),
        }
    }
}

// Div with T.

// Owned & owned.
impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

// Owned & borrowed.
impl Div<&Geo> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: &Geo) -> Self::Output {
        Vec2 {
            x: self.x.div(*rhs),
            y: self.y.div(*rhs),
        }
    }
}

// Borrowed & owned.
impl Div for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

// Borrowed & borrowed.
impl Div<&Geo> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: &Geo) -> Self::Output {
        Vec2 {
            x: self.x.div(*rhs),
            y: self.y.div(*rhs),
        }
    }
}

// Owned.
impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x.add_assign(other.x);
        self.y.add_assign(other.y);
    }
}

// Borrowed.
impl AddAssign<&Vec2> for Vec2 {
    fn add_assign(&mut self, other: &Vec2) {
        self.x.add_assign(other.x);
        self.y.add_assign(other.y);
    }
}

// SubAssign.

// Owned.
impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

// Borrowed.
impl SubAssign<&Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: &Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

// MulAssign with T.

// Owned.
impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

// Borrowed.
impl MulAssign<&Geo> for Vec2 {
    fn mul_assign(&mut self, rhs: &Geo) {
        self.x.mul_assign(*rhs);
        self.y.mul_assign(*rhs);
    }
}

// DivAssign with T.

// Owned.
impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

// Borrowed.
impl DivAssign<&Geo> for Vec2 {
    fn div_assign(&mut self, rhs: &Geo) {
        self.x.div_assign(*rhs);
        self.y.div_assign(*rhs);
    }
}