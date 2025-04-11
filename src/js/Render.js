import RenderMethods from "./RenderMethods";

export class Render {
	canvas;
	ctx;

	constructor(options = {}) {
		// ...do something with options in future
		
		let canvas = this.canvas = document.createElement("canvas");
		this.ctx = canvas.getContext("2d");

		canvas.style.position = "absolute";
		canvas.style.top = "0px";
		canvas.style.left = "0px";
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;

		window.addEventListener("resize", () => {
			canvas.width = window.innerWidth;
			canvas.height = window.innerHeight;
		});

		document.body.appendChild(canvas);
	}
	
	pair(x, y) {
		if (x > y) {
			return x * x + x + y;
		}
		return y * y + x
	}
	render(engine) {
		let { canvas, ctx } = this;

		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.save();

		let bodyIds = engine.world_get_bodies();
		// let pairs = engine.world_get_collision_pairs();
		let pairs = [];
		let collidingBodies = pairs.flatMap(pair => [pair.body_a, pair.body_b]);

		// Render colliding bodies filled in
		ctx.beginPath();
		for (let id of bodyIds) {
			if (collidingBodies.includes(id)) {
				RenderMethods.polygon(engine.body_get_vertices(id), ctx);
			}
		}
		ctx.fillStyle = "#4FC2B580";
		ctx.fill();
		
		// Render body outlines
		ctx.beginPath();
		for (let id of bodyIds) {
			RenderMethods.polygon(engine.body_get_vertices(id), ctx);
		}
		ctx.strokeStyle = "#4FC2B5";
		ctx.lineWidth = 1.5;
		ctx.stroke();

		// Render collision points
		for (let pair of pairs) {
			let { body_a, body_b, contacts, depth, normal, tangent, normal_point } = pair;
			if (contacts.length <= 0) continue;

			for (let contact of contacts) {
				let { incident, reference, vertex } = contact;
				ctx.beginPath();
				ctx.arc(vertex.x, vertex.y, 3, 0, Math.PI * 2);
				ctx.fillStyle = "white";
				ctx.fill();
			}

			ctx.beginPath();
			ctx.moveTo(normal_point.x, normal_point.y);
			ctx.lineTo(normal_point.x + normal.x * 10, normal_point.y + normal.y * 10);
			ctx.moveTo(normal_point.x, normal_point.y);
			ctx.lineTo(normal_point.x + tangent.x * 10, normal_point.y + tangent.y * 10);
			ctx.strokeStyle = "#DF7157";
			ctx.lineWidth = 3;
			ctx.stroke();
		}

		ctx.restore();
	}
}