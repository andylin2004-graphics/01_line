all:
	cargo run --manifest-path hw-01/Cargo.toml
	imagemagick open imageFile.ppm

art:
	cargo run --manifest-path hw-01-art/Cargo.toml

clean:
	cargo clean --manifest-path hw-01/Cargo.toml