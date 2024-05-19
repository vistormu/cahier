BINARY_NAME=cahier
DIST_DIR=./dist

default: build

create-dist:
	mkdir -p $(DIST_DIR)

# build: create-dist build-mac-x86 build-mac-arm64
build: create-dist build-linux-x86 build-linux-arm64

# build-mac-x86:
# 	cargo build --release --target x86_64-apple-darwin
# 	cp target/x86_64-apple-darwin/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME)-darwin-x86_64

# build-mac-arm64:
# 	cargo build --release --target aarch64-apple-darwin
# 	cp target/aarch64-apple-darwin/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME)-darwin-arm64

build-linux-x86:
	cross build --release --target x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME)-linux-x86_64

build-linux-arm64:
	cross build --release --target aarch64-unknown-linux-gnu
	cp target/aarch64-unknown-linux-gnu/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME)-linux-arm64

clean:
	cargo clean
	rm -rf $(DIST_DIR)

