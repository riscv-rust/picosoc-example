TARGET = riscv32imac-unknown-none-elf
BIN = rust-picosoc

FILE = ./target/$(TARGET)/debug/$(BIN)

all: firmware.hex firmware.bin

.PHONY: $(FILE)
$(FILE):
	cargo build

firmware.hex: $(FILE)
	riscv32-unknown-elf-objcopy -O verilog $< firmware.hex

firmware.bin: $(FILE)
	riscv32-unknown-elf-objcopy -O binary $< firmware.bin

.PHONY: clean
clean:
	rm -f firmware.hex firmware.bin
