import * as wasm from "wasm";
export * from "wasm";

export async function run() {
	const { Body, Vec2 } = wasm;
	console.log(wasm);

	// create canvas
	let canv = document.createElement("canvas");
	document.body.appendChild(canv);

	function updateCanvSize() {
		canv.width = window.innerWidth;
		canv.height = window.innerHeight;
	}
	updateCanvSize();
	window.addEventListener("resize", updateCanvSize);

	let ctx = canv.getContext("2d");
	
	let bodies = [];
	bodies.push(Body.rectangle(100, 100, Vec2.new(0, 0)));
	console.log(bodies[0]);

	
	tick();
	
	function tick() {
		render();
		requestAnimationFrame(tick);
	}
	function render() {
		ctx.clearRect(0, 0, canv.width, canv.height);

		ctx.save();
		ctx.translate(canv.width / 2, canv.height / 2);

		for (let body of bodies) {
			ctx.beginPath();
			let vertices = body.get_vertices();
			ctx.moveTo(vertices[0].x, vertices[0].y);
			for (let i = 1; i < vertices.length; ++i) {
				ctx.lineTo(vertices[i].x, vertices[i].y);
			}
			ctx.fillStyle = "#FD927C";
			ctx.fill();
		}

		ctx.restore();
	}
}