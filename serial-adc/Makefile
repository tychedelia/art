HEX:=serial-adc.hex

$(HEX):
	cargo objcopy --release -- -O ihex $@

.PHONY: check
check:
	cargo check

.PHONY: build
build: $(HEX)

.PHONY: install
install:
	./teensy_loader_cli.exe --mcu=TEENSY41 $(HEX)

