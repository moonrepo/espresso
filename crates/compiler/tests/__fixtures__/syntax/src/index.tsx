export class ClassName {
	field: number = 123;
	#private: boolean = true;

	method() {
		this.#private = false;
	}
	anonMethod = () => {
		this.field = 456_789;
	};

	static field: string = 'abc';

	static {
		this.field = 'xyz';
	}
}

export async function asyncFunc(...args: unknown[]) {
	return Promise.resolve();
}

export function* genFunc(value: string = 'abc') {
	yield 1;
}

export async function* asyncGenFunc() {
	yield 'abc';
}

const obj = { a: 1, b: 2, c: 3 };

const { a, b, c } = obj;
const [d, e, f] = [1, 2, 3];

const nullable = 1 ?? 2 ?? 3;
const optional = obj?.a || 0;

let assign = 0;
assign ||= a;
assign &&= b;
assign ??= b;

try {
	await import('path');
} catch {
	// Hrmmm
}

export function Component() {
	return <div />;
}
