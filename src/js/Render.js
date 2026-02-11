import RenderMethods from "./RenderMethods";

export class Render {
	canvas;
	ctx;

	position = { x: 0, y: 0 };
	scale = 1;

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
	unpair(n) {
		let sqrtz = Math.floor(Math.sqrt(n));
		let sqz = sqrtz * sqrtz;
		let result1 = ((n - sqz) >= sqrtz) ? { x: sqrtz, y: n - sqz - sqrtz } : { x: n - sqz, y: sqrtz };
		let x = result1.x % 2 === 0 ? result1.x / 2 : (result1.x + 1) / -2;
		let y = result1.y % 2 === 0 ? result1.y / 2 : (result1.y + 1) / -2;
		return { x: x, y: y };
	}
	render(engine) {
		let { canvas, ctx } = this;

		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.save();

		ctx.scale(this.scale, this.scale);
		ctx.translate(this.position.x, this.position.y);

		const renderBounds = false;
		const renderCollisions = false;
		const renderPairs = false;
		const renderGrid = true;
		const renderBodyIds = true;

		let bodyIds = engine.world_get_bodies();
		let pairs = renderPairs ? engine.world_get_collision_pairs() : [];
		let collidingBodies = pairs.flatMap(pair => [pair.body_a, pair.body_b]);
		let grid = renderGrid ? engine.world_get_grid() : {};
		
		// Render broadphase grid
		if (renderGrid) {
			const gridSize = 100; // from engine code; TODO: get this value from engine
			const margin = 2;

			// console.log(grid);
			
			ctx.fillStyle = "#ffe78920";
			ctx.strokeStyle = "#ffe78980";
			ctx.lineWidth = 2;
			
			for (let bucketId in grid) {
				ctx.beginPath();
				let position = this.unpair(bucketId);
				let bucketPosition = { x: position.x * gridSize, y: position.y * gridSize };

				ctx.rect(bucketPosition.x + margin, bucketPosition.y + margin, gridSize - 2*margin, gridSize - 2*margin);
				ctx.globalAlpha = Math.min(1, grid[bucketId] / 10);
				ctx.fill();
				ctx.stroke();

				// console.log(position.x, position.y, grid[bucketId]);
			}

			ctx.globalAlpha = 1;
		}

		// Render body bounds
		if (renderBounds) {
			ctx.beginPath();
			for (let id of bodyIds) {
				let bounds = engine.body_get_bounds(id);
				let width = bounds.max.x - bounds.min.x;
				let height = bounds.max.y - bounds.min.y;
				RenderMethods.roundedRect(width, height, { x: bounds.min.x + width/2, y: bounds.min.y + height/2 }, 0, ctx);
			}
			ctx.strokeStyle = "#ffcf8152";
			ctx.lineWidth = 1;
			ctx.stroke();
		}

		// Render colliding bodies filled in
		if (renderCollisions) {
			ctx.beginPath();
			for (let id of bodyIds) {
				if (collidingBodies.includes(id)) {
					RenderMethods.polygon(engine.body_get_vertices(id), ctx);
				}
			}
			ctx.fillStyle = "#4FC2B5A0";
			ctx.fill();
		}
		
		// Render body outlines
		ctx.beginPath();
		for (let id of bodyIds) {
			let vertices = engine.body_get_vertices(id);
			if (renderBodyIds) {
				let center = (() => {
					let v = new Vec2(0, 0);
					for (let x of vertices) {
						v.x += x.x;
						v.y += x.y;
					}
					v.x /= vertices.length;
					v.y /= vertices.length;
					return v;
				})();
				ctx.globalAlpha = 1;
				let a = ctx.fillStyle;
				ctx.textAlign = "center";
				ctx.fillStyle = "#FFFFFF";
				ctx.font = "16px Verdana";
				ctx.fillText(id, center.x, center.y + 7)
				ctx.fillStyle = a;
			}

			RenderMethods.polygon(vertices, ctx);
		}
		ctx.strokeStyle = "#4FC2B5";
		ctx.lineWidth = 1.5;
		ctx.stroke();

		// Render collision points
		if (renderPairs) {
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
		}

		ctx.restore();
	}
}