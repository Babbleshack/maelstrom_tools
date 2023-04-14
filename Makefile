MAELSTROM=maelstrom

.PHONY: echo clean fixup fmt lint


echo: ./target/debug/echo
	@echo "READY"

./target/debug/echo:
	cargo build --bin echo

clean:
	cargo clean

fixup: lint fmt;

fmt:
	cargo fmt
	
lint:
	cargo clippy --fix
