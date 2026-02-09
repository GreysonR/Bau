use crate::{Body, BodyMap, Geo, Vec2, World, collision_pair::{CollisionPair, Contact}};

pub fn find(world: &mut World, bodies: &mut BodyMap) {
	let pairs = world.find_pairs(bodies);

	for pair in pairs {
		let body_a = bodies.get(&pair.0).unwrap();
		let body_b = bodies.get(&pair.1).unwrap();
		if !collides(body_a, body_b) { continue; }

		// Create manifold (collision_pair) if they collide and add to world pairs (if it doesn't already exist)
		// let pair_id = CollisionPair::pair_id(body_a.id, body_b.id);
		world.collision_pairs.replace(create_manifold(world, body_a, body_b));
	}
}

fn collides(body_a: &Body, body_b: &Body) -> bool {
	// SAT: check if 1d projection has a collision for every axis of each body
	let find_supports = |body: &Body, direction: &Vec2| {
		let mut min = Geo::MAX;
		let mut max = Geo::MIN;
		for vertex in body.get_vertices().iter() {
			let proj = vertex.dot(direction);
			if proj < min { min = proj };
			if proj > max { max = proj };
		}
		(min, max)
	};
	let collides_against = |body_a: &Body, body_b: &Body| { // Checks body_a axes against body_b
		for axis in body_a.axes.iter() {
			let support_dir = axis.clone().normal();
			let a = find_supports(body_a, &support_dir);
			let b = find_supports(body_b, &support_dir);
			if a.1 < b.0 || a.0 > b.1 { // bounds aren't overlapping
				return false;
			}
		}
		true
	};
	collides_against(body_a, body_b) && collides_against(body_b, body_a)
}

fn create_manifold(world: &World, body_a: &Body, body_b: &Body) -> CollisionPair {
	/*
		Find collision points
			Check all vertices of both bodies for point overlapping
		Find collision normal
			Find edge with minimum penetration
				Penetration guaranteed to be positive (colliding) since if it wasn't it wouldn't pass collision test
				Penetration is (a.min - b.max).abs().min((a.max - b.min).abs()), where a and b are projected sides
		Find collision penetration (see above)
	*/
	let mut contacts = Vec::new();
	let mut depth: Geo = Geo::MAX;
	let mut normal = Vec2::zero();
	let mut normal_point = Vec2::zero();
	let mut reference_body_id = body_a.id;
	let mut incident_body_id = body_b.id;

	/*
	For each body:
		Iterate through all vertices
			Find min collision depth:
				Find edge
				Find edge normal
				Get support (depth, vertice index) along that normal
				If the depth < minimum found depth:
					update min depth, normal, and vertex index

			Find contacts:
				Find if current vertex is inside other body
				If it is, add it to contacts
					Incident body: body that owns vertex
					Reference body: body that is overlapping with verex
	*/

	let bodies = [body_b, body_a];
	for i in 0..bodies.len() {
		let incident = bodies[i];
		let reference = bodies[bodies.len() - i - 1];
		let vertices = incident.get_vertices();
		let reference_vertices = reference.get_vertices();

		for i in 0..vertices.len() {
			let cur_vertex = &vertices[i];
			let next_vertex = &vertices[(i + 1) % vertices.len()];
			// Find min collision depth / other manifold data
			let edge = next_vertex - cur_vertex;
			let support_normal = -edge.clone().normal().normalize();
			let support_index = reference.get_support(&support_normal);
			let support_depth = support_normal.dot(&(&reference_vertices[support_index] - cur_vertex));
			
			if support_depth < depth {
				depth = support_depth;
				normal = -support_normal;
				normal_point = cur_vertex + &edge * 0.5;
				
				reference_body_id = reference.id;
				incident_body_id = incident.id;
			}

			// Find overlapping contacts
			if reference.contains_point(&cur_vertex) {
				contacts.push(Contact {
					vertex: cur_vertex.clone(),
					reference: reference.id,
					incident: incident.id,
					anchor_a: Vec2::zero(),
					anchor_b: Vec2::zero(),
					mass_coefficient: 1.0,
				});
			}
		}
	}

	let reference_body = if body_a.id == reference_body_id { body_a } else { body_b };
	let incident_body = if body_a.id == incident_body_id { body_a } else { body_b };
	let mass_coef = 1.0 / contacts.len() as Geo;
	for contact in contacts.iter_mut() {
		let vertex = &contact.vertex.clone();
		contact.anchor_a = (vertex - reference_body.get_position()).rotate(-reference_body.get_angle());
		contact.anchor_b = (vertex - incident_body.get_position()).rotate(-incident_body.get_angle());
		contact.mass_coefficient = mass_coef;
	}


	CollisionPair {
		body_a: reference_body_id,
		body_b: incident_body_id,
		frame: world.frame,

		contacts,

		depth,
		tangent: normal.clone().normal(),
		normal,
		normal_point,

		friction: (body_a.friction.powi(2) + body_b.friction.powi(2)).sqrt(),
		restitution: 1.0 + body_a.restitution.max(body_b.restitution),
	}
}