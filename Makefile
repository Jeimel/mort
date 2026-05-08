export RUSTFLAGS := -C target-cpu=native

EXE := mort

build:
	cargo rustc --release -- --emit link=target/release/$(EXE)

pgo:
	cargo pgo instrument
	cargo pgo run -- bench
	cargo pgo optimize
