CC=cargo
CFLAGS=--release --target=x86_64-pc-windows-gnu --verbose

program2: program2/src/main.rs program2/src/funcs/types.rs program2/src/funcs.rs
	cd program2; \
	$(CC) build $(CFLAGS)