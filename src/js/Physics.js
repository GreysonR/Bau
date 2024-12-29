import { Physics as WasmPhysics } from "wasm";

export default class Physics {
	constructor(Engine) {
		this.Physics = WasmPhysics.new();
		this.Engine = Engine;
	}
	update() {
		this.Physics.update(this.Engine.World.World); // copies world?
	}
	free() {
		this.Physics.free();
	}
}