
.PHONY: test release


watch:
	axe $(SRCS) README.md -- cargo test

test:
	cargo test

release:
	cargo release
