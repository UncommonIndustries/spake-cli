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
	TARGET_CC=clang TARGET_AR=llvm-ar cargo build --verbose --release --target aarch64-apple-darwin
	mkdir -p ./dist/mac-arm
	cp ./target/aarch64-apple-darwin/release/$(NAME) ./dist/mac-arm/$(NAME)
	tar -czvf ./dist/mac-arm/$(NAME)-mac-arm64.tar.gz ./dist/mac-arm/$(NAME)

mac-x86-dist:
	rustup target add x86_64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	mkdir -p ./dist/mac-x86
	cp ./target/x86_64-apple-darwin/release/$(NAME) ./dist/mac-x86/$(NAME)
	tar -czvf ./dist/mac-x86/$(NAME)-mac-x86_64.tar.gz ./dist/mac-x86/$(NAME)
	
linux-x86_64-dist:
	rustup target add x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p ./dist/linux-x86_64
	cp ./target/x86_64-unknown-linux-gnu/release/$(NAME) ./dist/linux-x86_64/$(NAME)
	tar -czvf ./dist/linux-x86_64/$(NAME).tar.gz ./dist/linux-x86_64/$(NAME)