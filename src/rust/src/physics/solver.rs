use crate::{Body, Id, World, Time};
use std::collections::HashMap;
use nohash_hasher::BuildNoHashHasher;

pub fn solve_velocity(world: &mut World, bodies: &mut HashMap<Id, Body, BuildNoHashHasher<Id>>, delta: Time) {
	/*
		For all collision pairs
			For all collision contacts
				Get relative velocity at that point
				Solve contraint rel vel >= 0 by applying velocity * impulse
				Update final vel / angular vel for 
	*/

	// Clear old collision pairs
	world.collision_pairs.retain(|pair| {
		if !pair.is_valid(world.frame) {
			return false;
		}

		let body_a = bodies.get(&pair.body_a).expect("failed to get body_a in solver::solve_velocity");
		let body_b = bodies.get(&pair.body_b).expect("failed to get body_b in solver::solve_velocity");
		let normal = &pair.normal;
		let tangent = &pair.tangent;
		let contacts = &pair.contacts;
		let depth = pair.depth;


		let angle_a = body_a.angle;
		let ma = body_a.inverse_mass;
		let ia = body_a.inverse_inertia;
		let pa = &body_a.position;
		let wa = body_a.angular_velocity;
		let va = &body_a.velocity;
		
		let angle_b = body_b.angle;
		let mb = body_b.inverse_mass;
		let ib = body_b.inverse_inertia;
		let pb = &body_b.position;
		let wb = body_b.angular_velocity;
		let vb = &body_b.velocity;

		let mut vfa = va.clone(); // final velocity
		let mut wfa = wa; // final angular velocity
		let mut vfb = vb.clone();
		let mut wfb = wb;

		let restitution = pair.restitution;
		let friction = pair.friction;


		let slop = 1.0;
		
		for contact in contacts.iter() {
			let ra = contact.anchor_a.rotate(angle_a); // contact radius a
			let rb = contact.anchor_b.rotate(angle_b); // contact radius b
			let vra = va + ra.cross_float(wa); // relative velocity for body A at contact
			let vrb = vb + rb.cross_float(wb); // relative velocity for body B at contact
			let vr = vrb - vra; // relative velocity between body A and body B at contact
			let vn = vr.dot(normal); // normal velocity
			let vt = vr.dot(tangent); // tangent velocity
			
			// Adjusted separation
			let adjusted_separation = depth + ((&rb + pb) - (&ra + pa)).dot(normal);

			// Separation
			let ds = (&rb + vb) - (va + &ra);
			let mut s = ds.dot(normal) * delta + adjusted_separation; // separation scalar
			if s < 0.0 { continue; }
			s = (s - slop * s.signum()).max(0.0); // allow a little penetration
			
			// Baumgarte stabilization
			let bias = s * 10.0;

			// Normal mass
			let rna = ra.cross(normal);
			let rnb = rb.cross(normal);
			let k_normal = ma + mb + (ia * rna * rna) + (ib * rnb * rnb);
			let normal_mass = if k_normal > 0.0 { 1.0 / k_normal } else { 0.0 };

			// Tangent mass
			let rta = ra.cross(tangent);
			let rtb = rb.cross(tangent);
			let k_tangent = ma + mb + (ia * rta * rta) + (ib * rtb * rtb);
			let tangent_mass = if k_tangent > 0.0 { 1.0 / k_tangent } else { 0.0 };

			// Impulses
			let mut normal_impulse = normal_mass * contact.mass_coefficient * (vn * restitution + bias);
			let mut tangent_impulse = -vt * tangent_mass;
			
			// Clamp impulses
			normal_impulse = normal_impulse.max(0.0);

			// Coulomb friction
			let max_tangent_impulse = normal_impulse * friction;
			tangent_impulse = tangent_impulse.min(max_tangent_impulse).max(-max_tangent_impulse);

			let p = normal * normal_impulse - tangent * tangent_impulse; // final impulse

			vfa += &p * ma;
			wfa += ra.cross(&p) * ia;
			vfb -= &p * mb;
			wfb -= rb.cross(&p) * ib;
		}

		// update bodies
		let body_a = bodies.get_mut(&pair.body_a).expect("failed to get body_a in solver::solve_velocity");
		if !body_a.is_static {
			body_a.set_velocity(vfa);
			body_a.angular_velocity = wfa;
		}

		let body_b = bodies.get_mut(&pair.body_b).expect("failed to get body_a in solver::solve_velocity");
		if !body_b.is_static {
			body_b.set_velocity(vfb);
			body_b.angular_velocity = wfb;
		}

		true // oh yeah this is a ::retain, almost forgot with all that stuff up there
	});
}