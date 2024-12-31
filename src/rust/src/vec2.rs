// Modified version of [vec2](https://crates.io/crates/vec2) that removes non standard stuff

use std::{mem, ops::*};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use crate::Geo;

#[wasm_bindgen]
#[derive(Clone, PartialEq, Default, Debug, Serialize,  Deserialize)]
pub struct Vec2 {
	pub x: Geo,
	pub y: Geo,
}


#[wasm_bindgen]
impl Vec2 {
	#[wasm_bindgen(constructor)]
	pub fn new(x: Geo, y: Geo) -> Self {
		Self { x, y }
	}
	pub fn zero() -> Self {
		Self { x: 0.0, y: 0.0 }
	}

	pub fn length(&self) -> Geo {
		(self.x * self.x + self.y * self.y).sqrt()
	}
	pub fn normalize(mut self) -> Vec2 {
		let len = self.length();
		self.x /= len;
		self.y /= len;
		self
	}
	pub fn dot(&self, rhs: &Vec2) -> Geo {
		self.x * rhs.x + self.y * rhs.y
	}
	pub fn cross(&self, rhs: &Vec2) -> Geo {
		self.x * rhs.y - self.y * rhs.x
	}
	pub fn normal(mut self) -> Vec2 {
		mem::swap(&mut self.x, &mut self.y);
		self.y = -self.y;
		self
	}
	pub fn min(self, rhs: Vec2) -> Vec2 {
		Self {
			x: self.x.min(rhs.x),
			y: self.y.min(rhs.y),
		}
	}
	pub fn max(self, rhs: Vec2) -> Vec2 {
		Self {
			x: self.x.max(rhs.x),
			y: self.y.max(rhs.y),
		}
	}
	pub fn clamp(self, min: Vec2, max: Vec2) -> Vec2 {
		Self {
			x: self.x.clamp(min.x, max.x),
			y: self.y.clamp(min.y, max.y),
		}
	}
}
impl From<&Vec2> for Vec2 {
	fn from(other: &Vec2) -> Vec2 {
		other.clone()
	}
}
impl From<&mut Vec2> for Vec2 {
	fn from(other: &mut Vec2) -> Vec2 {
		other.clone()
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

impl Into<(Geo, Geo)> for Vec2 {
	fn into(self) -> (Geo, Geo) {
		(self.x, self.y)
	}
}

// Unary operators.

// Neg.

impl Neg for &Vec2 {
	type Output = Vec2;

	fn neg(self) -> Self::Output {
		Vec2 {
			x: self.x.neg(),
			y: self.y.neg(),
		}
	}
}
impl Neg for &mut Vec2 {
	type Output = Vec2;

	fn neg(self) -> Self::Output {
		Vec2 {
			x: self.x.neg(),
			y: self.y.neg(),
		}
	}
}
impl Neg for Vec2 {
	type Output = Vec2;
	fn neg(self) -> Self::Output { -&self }
}


// Binary operators.

// Add.
macro_rules! impl_add {
	($($lhs:ty, $rhs:ty),*) => {
		$(
			impl Add<$rhs> for $lhs {
				type Output = Vec2;
				
				fn add(self, rhs: $rhs) -> Self::Output {
					Vec2 {
						x: self.x.add(rhs.x),
						y: self.y.add(rhs.y),
					}
				}
			}
		)*
	};
}
impl_add!(
	Vec2, Vec2,
	Vec2, &Vec2,
	Vec2, &mut Vec2,
	&Vec2, Vec2,
	&Vec2, &Vec2,
	&Vec2, &mut Vec2,
	&mut Vec2, Vec2,
	&mut Vec2, &Vec2,
	&mut Vec2, &mut Vec2
);

// Sub.
macro_rules! impl_sub {
	($($lhs:ty, $rhs:ty),*) => {
		$(
			impl Sub<$rhs> for $lhs {
				type Output = Vec2;
				
				fn sub(self, rhs: $rhs) -> Self::Output {
					Vec2 {
						x: self.x.sub(rhs.x),
						y: self.y.sub(rhs.y),
					}
				}
			}
		)*
	};
}
impl_sub!(
	Vec2, Vec2,
	Vec2, &Vec2,
	Vec2, &mut Vec2,
	&Vec2, Vec2,
	&Vec2, &Vec2,
	&Vec2, &mut Vec2,
	&mut Vec2, Vec2,
	&mut Vec2, &Vec2,
	&mut Vec2, &mut Vec2
);

// Mul with T.
macro_rules! impl_mul {
	($($lhs:ty, $rhs:ty),*) => {
		$(
			impl Mul<$rhs> for $lhs {
				type Output = Vec2;
				
				fn mul(self, rhs: $rhs) -> Self::Output {
					Vec2 {
						x: self.x.mul(rhs.x),
						y: self.y.mul(rhs.y),
					}
				}
			}
		)*
	};
}
impl_mul!(
	Vec2, Vec2,
	Vec2, &Vec2,
	Vec2, &mut Vec2,
	&Vec2, Vec2,
	&Vec2, &Vec2,
	&Vec2, &mut Vec2,
	&mut Vec2, Vec2,
	&mut Vec2, &Vec2,
	&mut Vec2, &mut Vec2
);
macro_rules! impl_mul_geo {
	($($lhs:ty, $rhs:ty),*) => {
		$(
			impl Mul<$rhs> for $lhs {
				type Output = Vec2;
				
				fn mul(self, rhs: $rhs) -> Self::Output {
					Vec2 {
						x: self.x * rhs,
						y: self.y * rhs,
					}
				}
			}
		)*
	};
}
impl_mul_geo!(
	Vec2, Geo,
	&Vec2, Geo,
	&mut Vec2, Geo,
	Vec2, &Geo,
	&Vec2, &Geo,
	&mut Vec2, &Geo
);

// Div
macro_rules! impl_div {
	($($lhs:ty, $rhs:ty),*) => {
		$(
			impl Div<$rhs> for $lhs {
				type Output = Vec2;
				
				fn div(self, rhs: $rhs) -> Self::Output {
					Vec2 {
						x: self.x.div(rhs.x),
						y: self.y.div(rhs.y),
					}
				}
			}
		)*
	};
}
impl_div!(
	Vec2, Vec2,
	Vec2, &Vec2,
	Vec2, &mut Vec2,
	&Vec2, Vec2,
	&Vec2, &Vec2,
	&Vec2, &mut Vec2,
	&mut Vec2, Vec2,
	&mut Vec2, &Vec2,
	&mut Vec2, &mut Vec2
);
macro_rules! impl_div_geo {
	($($lhs:ty, $rhs:ty),*) => {
		$(
			impl Div<$rhs> for $lhs {
				type Output = Vec2;
				
				fn div(self, rhs: $rhs) -> Self::Output {
					Vec2 {
						x: self.x / rhs,
						y: self.y / rhs,
					}
				}
			}
		)*
	};
}
impl_div_geo!(
	Vec2, Geo,
	&Vec2, Geo,
	&mut Vec2, Geo,
	Vec2, &Geo,
	&Vec2, &Geo,
	&mut Vec2, &Geo
);

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
impl MulAssign<Geo> for Vec2 {
	fn mul_assign(&mut self, rhs: Geo) {
		self.x.mul_assign(rhs);
		self.y.mul_assign(rhs);
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
impl DivAssign<Geo> for Vec2 {
	fn div_assign(&mut self, rhs: Geo) {
		self.x.div_assign(rhs);
		self.y.div_assign(rhs);
	}
}

// Borrowed.
impl DivAssign<&Geo> for Vec2 {
	fn div_assign(&mut self, rhs: &Geo) {
		self.x.div_assign(*rhs);
		self.y.div_assign(*rhs);
	}
}