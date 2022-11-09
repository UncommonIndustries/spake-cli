NAME := spake-cli

build: 
	cargo build

test: build
	./target/debug/$(NAME) translate --host http://localhost:8000 --path ./tests/strings_en.json -t de

clean: 
	cargo clean
	-rm -rf dist/

artifacts: mac-arm-dist mac-x86-dist linux-x86_64-dist 

mac-arm-dist:
	rustup target add aarch64-apple-darwin
	cargo build  --release --target aarch64-apple-darwin
	mkdir -p dist/mac-arm
	tar -czvf dist/mac-arm/$(NAME)-mac-arm64.tar.gz -C ./target/aarch64-apple-darwin/release/ spake-cli

mac-x86_64-dist:
	rustup target add x86_64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	mkdir -p ./dist/mac-x86_64
	tar -czvf ./dist/mac-x86_64/$(NAME)-mac-x86_64.tar.gz -C ./target/x86_64-apple-darwin/release/ spake-cli

	
linux-x86_64-dist:
	rustup target add x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p ./dist/linux-x86_64
	cp ./target/x86_64-unknown-linux-gnu/release/$(NAME) ./dist/linux-x86_64/$(NAME)
	tar -czvf ./dist/linux-x86_64/$(NAME)_linux-x86_64.tar.gz ./dist/linux-x86_64/$(NAME)