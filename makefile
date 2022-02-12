all:
	cargo run --manifest-path hw-01/Cargo.toml
	magick display imageFile.ppm

art:
	cargo run --manifest-path hw-01-art/Cargo.toml

clean:
	cargo clean --manifest-path hw-01/Cargo.toml