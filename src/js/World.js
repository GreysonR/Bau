import { World as WasmWorld, Body, Vec2 } from "wasm";

export default class World {
	constructor() {
		this.World = WasmWorld.new();
	}
	addBody(body) { // temp1.World.addBody({ vertices: [{x: 0, y: 0}, {x: 100, y: 0}, {x: 100, y: 100}, ], position: { x: 0, y: 0 } });
		body = Body.new(body.vertices.map(v => Vec2.new(v.x, v.y)), Vec2.new(body.position.x, body.position.y));
		this.World.add_body(body);
	}
	free() {
		this.World.free();
	}
}