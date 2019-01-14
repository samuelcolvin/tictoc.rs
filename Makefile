.DEFAULT_GOAL := build

.PHONY: build
build:
	@mkdir -p build
	rustc -C opt-level=3 tic.rs
	@mv tic build/
	rustc -C opt-level=3 toc.rs
	@mv toc build/
