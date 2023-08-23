function a(constructor: Function) {}

function b(arg: string) {
	return (constructor: Function) => {};
}

function c() {
	return (target: any, propertyKey: string, descriptor: PropertyDescriptor) => {};
}

function d(arg: string) {
	return (target: any, propertyKey: string, descriptor: PropertyDescriptor) => {};
}

function format(msg: string) {
	return (target: any, propertyKey: string) => {};
}

function required(
	target: Object,
	propertyKey: string | symbol | undefined,
	parameterIndex: number,
) {}

@a
@b('arg')
export class Foo {
	@format('Hello, %s')
	greeting: string = '';

	@c()
	@d('arg')
	method(@required verbose: boolean) {}
}
