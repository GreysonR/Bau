import { polygon as renderPolygon } from "./RenderMethods";

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
	render(engine) {
		let { canvas, ctx } = this;

		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.save();

		let bodies = engine.get_bodies_vertices();
		ctx.beginPath();
		for (let body of bodies) {
			renderPolygon(body, ctx);
		}
		ctx.fillStyle = "#4FC2B5";
		ctx.strokeStyle = "#4FC2B5";
		ctx.lineWidth = 2;
		// ctx.fill();
		ctx.stroke();

		ctx.restore();
	}
}