fmt:
	cargo fmt --all

doc-open:
	cargo doc --all-features --open

doc:
	cargo doc --all-features

check:
	cargo check --all-features

test-all-features-tarpaulin:
	cargo tarpaulin --all-features --out Html --output-dir page-hunter/tests

test-all-features:
	cargo test --all-features