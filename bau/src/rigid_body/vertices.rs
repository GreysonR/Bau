use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Vertices {
	vertices: Vec<Vec2>
}

impl Vertices {
	pub fn new(vertices: Vec<Vec2>) -> Self {
		let center = Vertices::find_center_of_mass(&vertices);

		Self {
			vertices: vertices.iter().map(|vertex| vertex - center).collect(),
		}
	}

	fn find_center_of_mass(vertices: &Vec<Vec2>) -> Vec2 {
		let mut centroid = Vec2::ZERO;
		let mut det = 0.0;
		let mut temp_det;
		let num_vertices = vertices.len();

		for i in 0..num_vertices {
			let cur = vertices[i];
			let next = vertices[(i + 1) % num_vertices];

			temp_det = cur.perp_dot(next);
			det += temp_det;

			centroid += (cur + next) * temp_det;
		}

		centroid /= 3.0 * det;
		centroid
	}
	pub fn get_center_of_mass(&self) -> Vec2 {
		Vertices::find_center_of_mass(&self.vertices)
	}
	pub fn get_area(&self) -> f32 {
		let vertices = &self.vertices;
		let mut area = 0.0;
		let len = vertices.len();
		for i in 0..len {
			area += vertices[i].perp_dot(vertices[(i + 1) % len]);
		}
		area * 0.5
	}
	pub fn get_inertia(vertices: &Vec<Vec2>, mass: f32) -> f32 {
		if mass == f32::INFINITY { return f32::INFINITY } // if mass if infinity, body is static, so no work to be done

		let len = vertices.len();

		let mut numerator = 0.0;
		let mut denominator = 0.0;

		for i in 0..len {
			let cur = vertices[i];
			let next = vertices[(i + 1) % len];
			let cross = next.perp_dot(cur);

			numerator += cross * (next.dot(next) + next.dot(cur) + cur.dot(cur));
			denominator += cross;
		}

		(mass / 6.0) * (numerator / denominator)
	}
	pub fn contains_point(&self, point: Vec2) -> bool {
		let vertices = &self.vertices;
		let len = vertices.len();
		for i in 0..len {
			let j = (i + 1) % len;
			let va = vertices[i];
			let vb = vertices[j];
			let axis = vb - va;

			let dist = axis.perp_dot(point - va);
			if dist < 0.0 {
				return false;
			}
		}
		true
	}
}