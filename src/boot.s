.global _start
.global _start_non_boot_core

.extern _TMP_BOOT_STACK_BOTTOM
.extern _BSS_START
.extern _BSS_END_EXCL

.section .text.boot

// only the boot core starts here
_start:
// OpenSBI passes hart id in a0 register
// Keep it saved in tp register for each core
        mv tp, a0
// Devicetree pointer is passed in a1
// Save it in order to pass to kinit later
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
        la sp, _TMP_BOOT_STACK_BOTTOM
        mv a0, t0
        // Jump to Rust code.
        j kinit

// debug print hart id, then 
// infinitely wait for events (aka "park the core").
_start_non_boot_core:
        // hart_id passed in a0
        // sp passed in a1
        mv tp, a0
        mv sp, a1
        j start_non_boot_core
