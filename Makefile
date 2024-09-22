TOOLCHAIN = riscv64-unknown-elf
OBJDUMP = $(TOOLCHAIN)-objdump
GDB = $(TOOLCHAIN)-gdb

KERNEL_ELF = target/riscv64gc-unknown-none-elf/release/jannos

QEMU_ARGS = -machine virt -serial stdio -display none -cpu rv64 -smp 4


.PHONY: build
build: 
	RUSTFLAGS=-Clink-arg=--script=link.ld cargo build --release

.PHONY: qemu
qemu: build
	qemu-system-riscv64 $(QEMU_ARGS) -kernel $(KERNEL_ELF)

.PHONY: qemu-gdb
qemu-gdb: build
	qemu-system-riscv64 $(QEMU_ARGS) -kernel $(KERNEL_ELF) -s -S

.PHONY: gdb
gdb:
	$(GDB) 

.PHONY: text
text:
	$(OBJDUMP) -d $(KERNEL_ELF)
