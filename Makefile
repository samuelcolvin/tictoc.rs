.DEFAULT_GOAL := build

.PHONY: build
build:
	@mkdir -p build
	rustc -C opt-level=3 -C lto -C panic=abort tic.rs
	@mv tic build/
	rustc -C opt-level=3 -C lto -C panic=abort toc.rs
	@mv toc build/
