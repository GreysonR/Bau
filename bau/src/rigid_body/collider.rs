use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
	pub vertices: Vec<Vec2>,
}
impl Collider {
	pub fn from_vertices(vertices: Vec<Vec2>) -> Self {
		assert!(vertices.len() >= 3, "RigidBody Collider must have at least 3 vertices");
		let mut collider = Self { vertices };

		// normalize vertices before returning
		let center = collider.get_center_of_mass();
		let _ = collider.vertices.iter_mut().map(|vertex| {
			*vertex -= center;
		});

		collider
	}
	pub fn rectangle(width: f32, height: f32) -> Self {
		let half_width = width / 2.0;
		let half_height = height / 2.0;
		Self {
			vertices: vec![
				Vec2::new(-half_width, -half_height),
				Vec2::new( half_width, -half_height),
				Vec2::new( half_width,  half_height),
				Vec2::new(-half_width,  half_height),
			]
		}
	}
	pub fn get_center_of_mass(&self) -> Vec2 {
		let mut centroid = Vec2::ZERO;
		let mut det = 0.0;
		let mut temp_det;
		let num_vertices = self.vertices.len();

		for i in 0..num_vertices {
			let cur = self.vertices[i];
			let next = self.vertices[(i + 1) % num_vertices];

			temp_det = cur.perp_dot(next);
			det += temp_det;

			centroid += (cur + next) * temp_det;
		}

		centroid /= 3.0 * det;
		centroid
	}
	pub fn get_area(&self) -> f32 {
		let mut area = 0.0;
		let len = self.vertices.len();
		for i in 0..len {
			area += self.vertices[i].perp_dot(self.vertices[(i + 1) % len]);
		}
		area * 0.5
	}
	pub fn get_inertia(&self, mass: f32) -> f32 {
		let vertices = &self.vertices;
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
}
impl Default for Collider {
	fn default() -> Self {
		// Default to 100x100 rectangle
		Self::from_vertices(vec![Vec2::ZERO, Vec2::new(100.0, 0.0), Vec2::new(100.0, 100.0), Vec2::new(0.0, 100.0)])
	}
}