all:
	cargo run --manifest-path hw-01/Cargo.toml
	magick display imageFile.ppm&
	magick display arts.ppm&

clean:
	cargo clean --manifest-path hw-01/Cargo.toml