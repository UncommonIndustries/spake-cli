NAME := spake-cli

build: 
	cargo build

test: build
	./target/debug/$(NAME) --path ./tests/strings_en.json
