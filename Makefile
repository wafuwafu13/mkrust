.PHONY: build
build:
	cargo build

.PHONY: mkrust
mkrust: build
	./target/debug/mkrust ${ARGS}
