.global _start
.extern _STACK_PTR

.section .text.boot

_start:
        // Only proceed on the boot core (0). Park it otherwise.
        // csrr a0, mhartid
        // la t0, 0
        // ld a1, 0(t0)
        // bne a0, a1, .L_parking_loop
        // If execution reaches here, it is the boot core.

.hello_asm:
        li a1, 0x10000000
        addi a0, x0, 0x68
        sb a0, (a1) # 'h'
        addi a0, x0, 0x65
        sb a0, (a1) # 'e'
        addi a0, x0, 0x6C
        sb a0, (a1) # 'l'
        addi a0, x0, 0x6C
        sb a0, (a1) # 'l'
        addi a0, x0, 0x6F
        sb a0, (a1) # 'o'
        addi a0, x0, 0x0A
        sb a0, (a1) # '\n'

// Prepare the jump to Rust code.
.L_prepare_rust:
        // Set the stack pointer.
        la sp, _STACK_PTR
        // Jump to Rust code.
        j kmain

// Infinitely wait for events (aka "park the core").
.L_parking_loop:
        wfi
        j .L_parking_loop
