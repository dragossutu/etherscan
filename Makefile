
build_linux: test_linux
	cross build --release --target=x86_64-unknown-linux-musl

test_linux:
	cross test --release --target=x86_64-unknown-linux-musl
