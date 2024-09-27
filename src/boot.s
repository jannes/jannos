.global _start
.global _park_me
.extern _STACK_BOTTOM
.extern _BSS_START
.extern _BSS_END_EXCL

.section .text.boot

// only the boot core starts here
_start:
// OpenSBI passes hart id in a0 register
// Keep it saved in tp register for each core
        mv tp, a0
// Devicetree pointer is passed in a1
// Save it in order to pass to kmain later
        mv t0, a1
// Initialize DRAM.
        la a0, _BSS_START
        la a1, _BSS_END_EXCL

.L_bss_init_loop:
        beq a0, a1, .L_prepare_rust
        sd zero, (a0)
        addi a0, a0, 8
        j .L_bss_init_loop

// Prepare the jump to Rust code.
.L_prepare_rust:
        // Set the stack pointer.
        la sp, _STACK_BOTTOM
        mv a0, t0
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
