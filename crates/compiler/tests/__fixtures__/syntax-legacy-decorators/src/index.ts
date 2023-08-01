@a
@b('arg')
export class Foo {
	@format('Hello, %s')
	greeting: string;

	@c()
	@d('arg')
	method(@required verbose: boolean) {}
}
