fmt:
	cargo fmt --all

doc-open:
	cargo doc --all-features --open

doc:
	cargo doc --all-features

check:
	cargo check --all-features

test-all-features:
	cargo tarpaulin --all-features --out Html --output-dir page-hunter/tests