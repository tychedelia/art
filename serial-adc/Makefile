HEX:=serial-adc.hex
SRC:=$(wildcard ./src/*.rs)

$(HEX): $(SRC)
	cargo objcopy --release -- -O ihex $@

.PHONY: check
check:
	cargo check

.PHONY: build
build: $(HEX)

.PHONY: install
install: build
	./teensy_loader_cli.exe --mcu=TEENSY41 $(HEX)

