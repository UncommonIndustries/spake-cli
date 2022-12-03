NAME := spake-cli

build: 
	cargo build

test: build
	./target/debug/$(NAME) translate --host http://localhost:8000 --path ./tests/strings_en.json -t de
	#./target/debug/$(NAME) translate --host http://localhost:8000 --path ./strings/strings_de.json -t en -s de
	
	#./target/debug/$(NAME) translate --host http://localhost:8000 --path ./tests/long/strings_en.json -t fr -s en
	#./target/debug/$(NAME) translate --host http://localhost:8000 --path ./strings/strings_fr.json -t it -s fr

clean: 
	cargo clean
	-rm -rf dist/

version: # note on macos you need to install the gnu version of sed and alias it.
	$(eval VERSION := $(shell git describe --tags | sed 's/v//g' ))
	sed -i 's/0.0.0/${VERSION}/g' Cargo.toml

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
	tar -czvf ./dist/linux-x86_64/$(NAME)_linux-x86_64.tar.gz -C ./target/x86_64-unknown-linux-gnu/release/ spake-cli
