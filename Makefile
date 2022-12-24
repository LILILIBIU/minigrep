main:
	cargo run abc bbb.txt
test:
	cargo test --lib
clean:
	rm -rf ./target/*
build:
	cargo build -r
	
