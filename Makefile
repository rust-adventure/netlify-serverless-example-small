.PHONY: build

build:
	@cargo install --path . --root bins --target x86_64-unknown-linux-musl
	@mkdir -p functions
	@cp bins/* functions/