NAME := spake-cli

build: 
	cargo build

test: build
	./target/debug/$(NAME) --path ./tests/strings_en.json

artifacts: mac-arm-dist mac-x86-dist 

mac-arm-dist:
	cargo build --release --target aarch64-apple-darwin
	mkdir -p ./dist/mac-arm
	cp ./target/aarch64-apple-darwin/release/$(NAME) ./dist/mac-arm/$(NAME)
	tar -czvf ./dist/mac-arm/$(NAME)-mac-arm64.tar.gz ./dist/mac-arm/$(NAME)

mac-x86-dist:
	cargo build --release --target x86_64-apple-darwin
	mkdir -p ./dist/mac-x86
	cp ./target/x86_64-apple-darwin/release/$(NAME) ./dist/mac-x86/$(NAME)
	tar -czvf ./dist/mac-x86/$(NAME)-mac-x86_64.tar.gz ./dist/mac-x86/$(NAME)

linux-x86_64-dist:
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p ./dist/linux-x86_64
	cp ./target/x86_64-unknown-linux-gnu/release/$(NAME) ./dist/linux-x86_64/$(NAME)
	tar -czvf ./dist/linux-x86_64/$(NAME).tar.gz ./dist/linux-x86_64/$(NAME)