export RUSTFLAGS := -C target-cpu=native

EXE := reckless

build:
	cargo rustc --release -- --emit link=target/release/$(EXE)

pgo:
	cargo pgo instrument
	cargo pgo run -- bench
	cargo pgo optimize
