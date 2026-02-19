use bevy::prelude::*;
use super::Body;
use std::f32::consts::PI;

#[allow(unused)] // TODO: maybe remove these allows
pub struct BodyBuilder { // hell yeah
	// Stateful properties
	vertices: Vec<Vec2>, // vertices must be convex and counter clockwise winding (CCW)
	position: Vec2,
	angle: f32,
	velocity: Vec2,
	
	// Inherent
	mass: f32,
	friction_air: f32,
	is_static: bool,
}
impl Default for BodyBuilder {
	fn default() -> Self {
		Self {
			vertices: Vec::new(),
			position: Vec2::ZERO,
			angle: 0.0,
			velocity: Vec2::ZERO,

			mass: 1.0,
			friction_air: 1.0,
			is_static: false,
		}
	}
}

#[allow(unused)]
impl BodyBuilder {
	pub fn from_vertices(vertices: Vec<Vec2>) -> Self {
		assert!(vertices.len() >= 3, "Body must have at least 3 vertices");

		Self {
			vertices,
			..Default::default()
		}
	}
	pub fn rect(width: f32, height: f32) -> Self {
		assert!(width > 0.0, "Width must be positive");
		assert!(height > 0.0, "Height must be positive");
		
		Self {
			vertices: vec![
				Vec2::new(-width / 2.0,  height / 2.0),
				Vec2::new(-width / 2.0, -height / 2.0),
				Vec2::new( width / 2.0, -height / 2.0),
				Vec2::new( width / 2.0,  height / 2.0),
			],
			..Default::default()
		}
	}
	pub fn circle(radius: f32) -> Self {
		assert!(radius > 0.0, "Radius must be positive");

		let vertex_count = (radius.powf(0.333) * 1.0).round() as usize;
		let delta_angle = PI * 2.0 / (vertex_count as f32); // angle between each vertex
		let mut vertices = Vec::new();
		vertices.reserve_exact(vertex_count);

		for i in 0..vertex_count {
			let angle = delta_angle * (i as f32) + delta_angle / 2.0;
			vertices.push(Vec2::new(
				angle.cos() * radius,
				angle.sin() * radius,
			));
		}

		Self {
			vertices,
			..Default::default()
		}
	}

	pub fn position(mut self, position: Vec2) -> Self { self.position = position; self }
	pub fn angle(mut self, angle: f32) -> Self { self.angle = angle; self }
	pub fn velocity(mut self, velocity: Vec2) -> Self { self.velocity = velocity; self }
	pub fn mass(mut self, mass: f32) -> Self { self.mass = mass; self }
	pub fn friction_air(mut self, friction_air: f32) -> Self { self.friction_air = friction_air; self }
	pub fn is_static(mut self, is_static: bool) -> Self { self.is_static = is_static; self }

	fn get_center_of_mass(&self) -> Vec2 {
		let mut centroid = Vec2::ZERO;
		let mut det = 0.0;
		let mut temp_det = 0.0;
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
	fn get_area(&self) -> f32 {
		let mut area = 0.0;
		let len = self.vertices.len();
		for i in 0..len {
			area += self.vertices[i].perp_dot(self.vertices[(i + 1) % len]);
		}
		area * 0.5
	}
	fn get_inertia(&self) -> f32 { // I found this algo somewhere in the great depths of the internet. It was ancient then and it's probably gone now, so I have preemptively given up trying to source it.
		if self.is_static { return f32::INFINITY }

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

		(self.mass / 6.0) * (numerator / denominator)
	}
	pub fn build(self) -> Body {
		let is_static = self.is_static;
		let inertia = self.get_inertia();
		let inverse_inertia = if is_static { 0.0 } else { 1.0 / inertia };
		let inverse_mass = if is_static { 0.0 } else { 1.0 / self.mass };

		// We need to transform the vertices from wherever they are to the desired position & angle
		let true_position = self.get_center_of_mass();
		let desired_position = self.position;
		let angle_vec = Vec2::from_angle(self.angle);
		let vertices = self.vertices
			.iter()
			.map(|original| (original - true_position).rotate(angle_vec) + desired_position)
			.collect();

		Body {
			vertices,
			position: self.position,
			velocity: self.velocity,

			mass: self.mass,
			friction_air: self.friction_air,
			is_static,

			inverse_mass,
			inertia,
			inverse_inertia,

			..Default::default() // don't init any solver properties
		}
	}
}
