all:
	cargo run --manifest-path hw-01/Cargo.toml
	magick display imageFile.ppm

clean:
	cargo clean --manifest-path hw-01/Cargo.toml