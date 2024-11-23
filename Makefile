.PHONY: test-lib

test-lib:
	@cargo test -p snapshots -- --show-output
