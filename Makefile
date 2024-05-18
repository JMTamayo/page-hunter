fmt:
	cargo fmt --all

doc-open:
	cargo doc --all-features --open

doc:
	cargo doc --all-features

check:
	cargo check --all-features

test-tarpaulin:
	cargo tarpaulin --all-features --out Html --output-dir page-hunter/tests

test-llvm-cov:
	cargo llvm-cov --html --workspace --all-features

test:
	cargo test --all-features