release:
	cargo build --release

linux-release:
	TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl