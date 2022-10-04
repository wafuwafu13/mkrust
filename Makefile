.PHONY: build
build:
	cargo build

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: mkrust
mkrust: build fmt
	./target/debug/mkrust ${ARGS}
