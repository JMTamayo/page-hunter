fmt:
	cargo fmt --all

doc-open:
	cargo doc --all-features --open

doc:
	cargo doc --all-features

check:
	cargo check --all-features

test-tarpaulin:
	export $(shell cat local.env | xargs) && cargo tarpaulin --all-features --out Html --output-dir page-hunter/tests

test-llvm-cov:
	export $(shell cat local.env | xargs) && cargo llvm-cov --html --workspace --all-features

test:
	export $(shell cat local.env | xargs) && cargo test --all-features