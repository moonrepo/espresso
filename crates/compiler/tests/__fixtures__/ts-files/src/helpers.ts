export async function asyncFunc() {
	await Promise.resolve();
}

export function* generatorFunc() {
	yield 1;
}
