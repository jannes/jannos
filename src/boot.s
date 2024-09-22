.global _start
.global _park_me
.extern _STACK_PTR

.section .text.boot

_start:
// OpenSBI passes hart id in a0 register
// Keep it saved in tp register for each core
        mv tp, a0

// Prepare the jump to Rust code.
.L_prepare_rust:
        // Set the stack pointer.
        la sp, _STACK_PTR
        // Jump to Rust code.
        j kmain

// debug print hart id, then 
// infinitely wait for events (aka "park the core").
_park_me:
        mv tp, a0
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
        mv a0, tp
        addi a0, a0, 48
        sb a0, (a1) # 'hartid'
        addi a0, x0, 0x0A
        sb a0, (a1) # '\n'
        li a1, 0
.L_parking_loop:
        wfi
        j .L_parking_loop
