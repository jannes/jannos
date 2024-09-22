.global _start
.extern _STACK_PTR

.section .text.boot

_start:
// OpenSBI passes hart id in a0 register
        csrw sscratch, a0
// debug print hart id
_hello_asm:
        li a1, 0x10000000
        addi a0, x0, 0x49
        sb a0, (a1) # 'I'
        addi a0, x0, 0x20
        sb a0, (a1) # ' '
        addi a0, x0, 0x61
        sb a0, (a1) # 'a'
        addi a0, x0, 0x6D
        sb a0, (a1) # 'm'
        addi a0, x0, 0x20
        sb a0, (a1) # ' '
        csrr a0, sscratch
        addi a0, a0, 48
        sb a0, (a1) # 'hartid'
        addi a0, x0, 0x0A
        sb a0, (a1) # '\n'
        li a1, 0
        bne a0, a1, .L_parked

// TODO: start other harts
         at parking loop

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
