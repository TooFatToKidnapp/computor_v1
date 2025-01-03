CARGO = cargo
PROJECT_NAME = computor
BIN_NAME = target/debug/$(PROJECT_NAME)

all: build

build:
	$(CARGO) build --release

clean:
	$(CARGO) clean

fclean: clean
	rm -f Cargo.lock

re: fclean all

test:
	$(CARGO) test

.PHONY: all build clean re test
