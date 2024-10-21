TARGETS = x86_64-apple-darwin aarch64-apple-darwin x86_64-unknown-linux-gnu x86_64-pc-windows-gnu

.PHONY: clean
clean:
	@cargo clean

.PHONY: update
update:
	@cargo update

.PHONY: build
build: .check_rust .check_cross
	@cross build --release

.PHONY: .check_rust
.check_rust:
	@command -v rustc > /dev/null 2>&1 || (echo "Rust not found, installing..." && $(MAKE) .install_rust)

.PHONY: .install_rust
install_rust:
	@echo "Installing Rust..."
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

.PHONY: .check_cross
.check_cross:
	@command -v cross > /dev/null 2>&1 || (echo "Cross not found, installing..." && $(MAKE) .install_cross)

.PHONY: .install_cross
.install_cross:
	@echo "Installing Cross..."
	@cargo install cross --git https://github.com/cross-rs/cross

.PHONY: all
all: .check_rust .check_cross $(TARGETS)

.PHONY: $(TARGETS)
$(TARGETS):
	cross build --target $@ --release
