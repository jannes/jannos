# jannos: jannes' learning project for OS dev

Building a kernel for Qemu's riscv virtual machine from scratch in Rust.

- uses OpenSBI, a sort of BIOS that Qemu defaults to, to simplify some peripheral interactions
- does not use any dependencies and as little assembly as possible

## Notes

OpenSBI selects a hart as boot core and jumps to kernel.
Kernel has to start up other harts by itself.

## References
- [https://osblog.stephenmarz.com/](https://osblog.stephenmarz.com/)
- TODO: add more
