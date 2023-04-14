MAELSTROM=maelstrom
MAELSTROM_TEST=$(MAELSTROM) test
DEBUG_DIR=./target/debug

ECHO_BIN=$(DEBUG_DIR)/echo

.PHONY: echo test_echo clean fixup fmt lint

test_echo: $(ECHO_BIN)
	$(MAELSTROM_TEST) --log-stderr --log-net-send --log-net-recv -w echo --bin $(ECHO_BIN) --nodes n1 --rate 1 --time-limit 3

echo: $(ECHO_BIN)
	$(MAELSTROM_TEST) -w echo --bin $(ECHO_BIN)

$(ECHO_BIN):
	cargo build --bin echo

clean:
	cargo clean

fixup: lint fmt;

fmt:
	cargo fmt
	
lint:
	cargo clippy --fix
