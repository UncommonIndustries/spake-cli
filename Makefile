NAME := spake-cli

build: 
	cargo build

test: build
	./target/debug/$(NAME) --path ./example.json
