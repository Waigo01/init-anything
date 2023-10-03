default: 
	cargo build --release

install:
	sudo cp ./target/release/init-anything /usr/bin/
	mkdir -p ~/.init-anything
	cp -r ./templates ~/.init-anything/
