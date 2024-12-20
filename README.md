# jannos: jannes' learning project for OS dev

Building a kernel for Qemu's riscv virtual machine from scratch in Rust.

- uses OpenSBI, a sort of BIOS that Qemu defaults to, to simplify some peripheral interactions
- I am trying to use as few dependencies and as little assembly as possible

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- riscv64 target: `rustup target add riscv64gc-unknown-none-elf`
- `qemu-system-riscv64` (e.g `sudo dnf install qemu-system-riscv` on Fedora)
- `riscv64-unknown-elf` toolchain (providing gdb and binutils)

To compile the latter from source see 
[riscv-gnu-toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain)

## build, run, debug
- `make build` to build the kernel ELF image
- `make text` dump dissassembled text section
- `make qemu` to run in qemu
- `make qemu-gdb` to run in debuggable qemu session
- `make gdb` to debug running debuggable qemu session

## Notes

OpenSBI selects a hart as boot core and jumps to kernel.
Kernel has to start up other harts by itself.

## References
- [https://github.com/mit-pdos/xv6-riscv](https://github.com/mit-pdos/xv6-riscv)
- [https://osblog.stephenmarz.com/](https://osblog.stephenmarz.com/)
- many more, TODO: add

