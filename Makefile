MAELSTROM=maelstrom
MAELSTROM_TEST=$(MAELSTROM) test
DEBUG_DIR=./target/debug
DEBUG_ARGS = --log-stderr --log-net-send --log-net-recv

ECHO_BIN=$(DEBUG_DIR)/echo
UNIQUE_ID_BIN=$(DEBUG_DIR)/unique_id

.PHONY: echo test_echo clean fixup fmt lint

test_echo: $(ECHO_BIN)
	$(MAELSTROM_TEST) --log-stderr --log-net-send --log-net-recv -w echo --bin $(ECHO_BIN) --nodes n1 --rate 1 --time-limit 3

echo: clean $(ECHO_BIN)
	$(MAELSTROM_TEST) -w echo --bin $(ECHO_BIN)

$(ECHO_BIN):
	cargo build --bin echo

#maelstrom test -w unique-ids --bin ~/go/bin/maelstrom-unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
unique_id: clean $(UNIQUE_ID_BIN)
	$(MAELSTROM_TEST) -w unique-ids --bin $(UNIQUE_ID_BIN) --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

unique_id_test: $(UNIQUE_ID_BIN)
	$(MAELSTROM_TEST) $(DEBUG_ARGS) -w unique-ids --bin $(UNIQUE_ID_BIN) --time-limit 3 --rate 1 --node-count 1

$(UNIQUE_ID_BIN):
	cargo build --bin unique_id


clean:
	cargo clean

fixup: lint fmt;

fmt:
	cargo fmt
	
lint:
	cargo clippy --fix
