TOOLCHAIN = riscv64-unknown-elf
OBJDUMP = $(TOOLCHAIN)-objdump
GDB = $(TOOLCHAIN)-gdb

BUILD ?= debug

ifeq ($(BUILD), debug)
	KERNEL_ELF = target/riscv64gc-unknown-none-elf/debug/jannos
	CARGO_FLAGS =
else 
	KERNEL_ELF = target/riscv64gc-unknown-none-elf/release/jannos
	CARGO_FLAGS = --release
endif

NUM_CPU ?= 4

QEMU_ARGS = -machine virt -serial stdio -display none -cpu rv64 -smp $(NUM_CPU)


.PHONY: build
build: 
	RUSTFLAGS=-Clink-arg=--script=link.ld cargo build $(CARGO_FLAGS)

.PHONY: qemu
qemu: build
	qemu-system-riscv64 $(QEMU_ARGS) -kernel $(KERNEL_ELF)

.PHONY: qemu-gdb
qemu-gdb: build
	qemu-system-riscv64 $(QEMU_ARGS) -kernel $(KERNEL_ELF) -s -S

.PHONY: gdb
gdb:
	$(GDB) $(KERNEL_ELF)

.PHONY: text
text:
	$(OBJDUMP) -d $(KERNEL_ELF)
