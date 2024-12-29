export default class Vec2 {
	x;
	y;
	constructor(x, y) {
		this.x = x;
		this.y = y;
	}
	/**
	 * Creates a Vec2 from an array such in the form [x, y]
	 * @param {Array} arr - Array to create from
	 * @return {Vec2}
	 */
	static fromArray(arr) {
		return new Vec2(arr[0], arr[1]);
	}
	/**
	 * Creates a Vec2 from an object with x/y fields, i.e. { x: 20, y: 7 }
	 * @param {Object} obj - Object with x/y fields
	 * @return {Vec2}
	 */
	static fromObject(obj) {
		return new Vec2(obj.x, obj.y);
	}

	add(other) {
		return new Vec2(this.x + other.x, this.y + other.y);
	}
	sub(other) {
		return new Vec2(this.x - other.x, this.y - other.y);
	}
	get length() {
		return Math.sqrt(this.x * this.x + this.y * this.y);
	}
	normalize() {
		let len = this.length;
		if (len == 0) return new Vec2(0, 0);
		return new Vec2(this.x / len, this.y / len);
	}
	mult(other) {
		if (typeof other === "number") {
			return new Vec2(this.x * other, this.y * other);
		}
		else {
			return new Vec2(this.x * other.x, this.y * other.y);
		}
	}
}