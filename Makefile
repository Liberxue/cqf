.PHONY: init
# init env
init:
	@echo "init"

.PHONY:fmt
# cargo fmt
fmt:
	@cargo fmt -v 

.PHONY:unitTest
# unit Test
test:
	@export RUST_LOG=info
	@cargo test -v &&  cargo test --workspace

.PHONY: build
# cargo build
build:
@ if [ "$(OS)" == "Windows_NT" ]; then \
    cargo clean && cargo check && cargo build --release --target=x86_64-pc-windows-gnu; \
elif [ "$(shell uname)" == "Darwin" ]; then \
    cargo clean && cargo check && cargo build --release --target=x86_64-apple-darwin; \
else \
    cargo clean && cargo check && cargo build --release --target=x86_64-unknown-linux-musl; \
fi

.PHONY: all
# generate all
all:
	@echo "make all done"

# show help
help:
	@echo ''
	@echo 'Usage:'
	@echo ' make [target]'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
	helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf "\033[36m%-22s\033[0m %s\n", helpCommand,helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)
.DEFAULT_GOAL := help
