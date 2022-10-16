.PHONY: build
build:
	cargo build

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: mkrust
mkrust: fmt
	cargo run ${ARGS}
