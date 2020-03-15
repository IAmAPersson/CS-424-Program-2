CC=cargo
CFLAGS=--release --verbose

program2: program2/src/main.rs program2/src/funcs/types.rs program2/src/funcs.rs
	cd program2; \
	$(CC) build $(CFLAGS)