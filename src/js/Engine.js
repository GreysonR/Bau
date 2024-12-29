import * as wasm from "wasm";

import World from "./World";
import Physics from "./Physics";
import Ticker from "./Ticker";
import Performance from "./Performance";

export class Engine {
	World;
	Physics;
	Ticker;
	Performance;

	constructor() {
		// const { Body, Vec2 } = wasm;
		// console.log(wasm);

		this.World = new World();
		this.Physics = new Physics(this);

		this.Performance = new Performance();
		this.Ticker = new Ticker(this);
	}
	tick() {
		this.Physics.update();
	}
	free() {
		this.World.free();
		this.Physics.free();
	}
}
