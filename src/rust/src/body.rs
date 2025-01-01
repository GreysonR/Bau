use core::f32;

use crate::{Vec2, Geo, Id};

mod body_options;
pub use body_options::BodyOptions;

pub struct Body {
	pub id: Id,

	// Inherent
	vertices: Vec<Vec2>,
	position: Vec2,
	angle: Geo,
	pub velocity: Vec2,
	pub angular_velocity: Geo,
	
	// Calculated
	pub axes: Vec<Vec2>,
	inertia: Geo,
	inverse_inertia: Geo,
	inverse_mass: Geo,
	
	// Options
	mass: Geo,
	pub is_static: bool,
}

impl Body {
	//
	// constructors
	//
	pub fn new(vertices: Vec<Vec2>, position: Vec2, options: BodyOptions) -> Body {
		assert!(vertices.len() >= 3); // There should be at least 3 vertices for a valid body
		
		let mut body = Body {
			id: rand::random(),

			axes: Body::get_axes(&vertices),

			vertices,
			position: Vec2::new(0.0, 0.0),
			velocity: Vec2::new(0.0, 0.0),
			angle: 0.0,
			angular_velocity: 0.0,

			mass: options.mass,
			inverse_mass: 1.0 / options.mass,
			is_static: options.is_static,

			inertia: 1.0,
			inverse_inertia: 1.0,
		};

		body.update_inertia();

		body.translate_position(position);

		body
	}
	pub fn rectangle(width: Geo, height: Geo, position: Vec2, options: BodyOptions) -> Body {
		let half_width = width / 2.0;
		let half_height = height / 2.0;
		let vertices = vec![
			Vec2::new(-half_width, -half_height), // top left
			Vec2::new( half_width, -half_height), // top right
			Vec2::new( half_width,  half_height), // bottom right
			Vec2::new(-half_width,  half_height), // bottom left
		];
		Body::new(vertices, position, options)
	}
	pub fn circle(radius: Geo, position: Vec2, options: BodyOptions) -> Body {
		let mut vertices = Vec::new();
		let vertice_count = (radius.powf(0.333) * 8.0).round() as u32;
		
		let start_angle = f32::consts::TAU * 2.0 / vertice_count as Geo;
		for i in 0..vertice_count {
			vertices.push(Vec2::new((start_angle * i as Geo + start_angle / 2.0).cos() * radius, (start_angle * i as Geo + start_angle / 2.0).sin() * radius));
		}
		Body::new(vertices, position, options)
	}

	// Helper methods
	fn update_inertia(&mut self) {
		self.inertia = Body::calculate_inertia(self);
		self.inverse_inertia = 1.0 / self.inertia;
	}

	//
	// property calculation
	//
	fn get_axes(vertices: &Vec<Vec2>) -> Vec<Vec2> {
		let mut axes = Vec::new();
		let len = vertices.len();
		for i in 0..len {
			let j = (i + 1) % len;
			let axis = (&vertices[j] - &vertices[i]).normalize();
			axes.push(axis);
		}
		axes
	}
	fn calculate_inertia(body: &Body) -> Geo {
		if body.is_static { return Geo::MAX; }
		let vertices = &body.vertices;
		let len = vertices.len();
		let mass = body.mass;
		
		let mut numerator = 0.0;
		let mut denominator = 0.0;

		for i in 0..len {
			let j = (i + 1) % len;
			let cur = &vertices[i];
			let next= &vertices[j];

			let cross = next.cross(cur).abs();
			numerator += cross * (vertices[j].dot(&vertices[j]) + vertices[j].dot(&vertices[i]) + vertices[i].dot(&vertices[i]));
			denominator += cross;
		}

		return (mass / 6.0) * (numerator / denominator);
	}
	
	//
	// getters
	// 
	pub fn get_angle(&self) -> Geo { self.angle }
	pub fn get_position(&self) -> &Vec2 { &self.position }
	pub fn get_vertices(&self) -> &Vec<Vec2> { &self.vertices }
	pub fn get_velocity(&self) -> &Vec2 { &self.velocity }
	pub fn get_mass(&self) -> Geo { self.mass }
	pub fn get_inverse_mass(&self) -> Geo { self.inverse_mass }
	pub fn get_inertia(&self) -> Geo { self.inertia }
	pub fn get_inverse_inertia(&self) -> Geo { self.inverse_inertia }

	//
	// setters
	//

	// position
	pub fn set_position(&mut self, position: Vec2) {
		self.translate_position(position - &self.position);
	}
	pub fn translate_position(&mut self, translation: Vec2) {
		self.position += &translation;
		
		for vertex in self.vertices.iter_mut() {
			*vertex += &translation;
		}
	}
	// velocity
	pub fn set_velocity(&mut self, velocity: Vec2) {
		self.velocity = velocity;
	}
	pub fn apply_velocity(&mut self, force: &Vec2) {
		self.velocity += force;
	}
	// angle
	pub fn translate_angle(&mut self, angle: Geo) {
		self.angle += angle;
		let sin = angle.sin();
		let cos = angle.cos();
		let position = &self.position;
		for vertice in self.vertices.iter_mut() {
			let dist = vertice.clone() - position;
			vertice.x = position.x + (dist.x * cos - dist.y * sin);
			vertice.y = position.y + (dist.x * sin + dist.y * cos);
		}
		self.axes = Body::get_axes(&self.vertices);
	}
	pub fn set_angle(&mut self, angle: Geo) {
		self.translate_angle(angle - self.angle);
		self.angle = angle;
	}
	pub fn apply_angular_velocity(&mut self, force: Geo) {
		self.angular_velocity += force;
	}

	// physics helper methods
	pub fn contains_point(&self, point: &Vec2) -> bool {
		let vertices = &self.vertices;
		for i in 0..vertices.len() {
			let cur_vertex = &vertices[i];
			let next_vertex = &vertices[(i + 1) % vertices.len()];
			
			// edge_normal: (next.y - cur.y, -(next.x - cur.x))
			// cur_to_point = point - cur
			// edge_normal dot cur_to_point >= 0, then point is outside body (similar to SAT)
			if (point.x - cur_vertex.x) * (next_vertex.y - cur_vertex.y) + (cur_vertex.x - next_vertex.x) * (point.y - cur_vertex.y) >= 0.0 {
				return false;
			}
		}

		true
	}
	pub fn get_support(&self, direction: &Vec2) -> usize {
		let vertices = &self.vertices;
		let mut farthest_dist: Geo = Geo::MIN; // farthest distance in direction
		let mut farthest_vert_index = 0; // farthest vertice index in direction
		for i in 0..vertices.len() {
			let dist = direction.dot(&vertices[i]);

			if dist > farthest_dist {
				farthest_dist = dist;
				farthest_vert_index = i;
			}
		}

		farthest_vert_index
	}
	pub fn get_supports(&self, direction: &Vec2) -> ((Geo, usize), (Geo, usize)) {
		let mut min = Geo::MAX;
		let mut min_index = 0;
		let mut max = Geo::MIN;
		let mut max_index = 0;
		for (i, vertex) in self.vertices.iter().enumerate() {
			let proj = vertex.dot(direction);
			if proj < min {
				min = proj;
				min_index = i;
			}
			if proj > max {
				max = proj;
				max_index = i;
			}
		}
		((min, min_index), (max, max_index))
	}
}

impl PartialEq for Body {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Body {}
