EXE = tictactoe-rust

all: install

build: 
	cargo build

release:
	cargo build --release

install: release
	cp target/release/${EXE} .
	@echo You have an executable!
	@echo Just run it with ./${EXE}
	@echo Computer v.s. computer ./${EXE} --CVC

run: install
	./${EXE}

clean:
	rm tictactoe-rust

zip:
	zip tictactoe-rust.zip src/* Cargo.toml Makefile README.md

.PHONY: build release install clean zip all run
