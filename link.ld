MEMORY { 
    /* qemu uses OPENSBI which jumps to this address */
    ram  (wxa) : ORIGIN = 0x80200000, LENGTH = 128M
}


SECTIONS {
    .text.boot : {
        *(.text.boot)
    } > ram

    .text : {
        *(.text .text.*)
    } > ram

    .data : {
        . = ALIGN(16);
        *(.sdata .sdata.*) 
        . = ALIGN(16);
        *(.data .data.*)
    } > ram

    .rodata : {
        . = ALIGN(16);
        *(.srodata .srodata.*)
        . = ALIGN(16);
        *(.rodata .rodata.*)
    } > ram

    .bss : {
        . = ALIGN(16);
        _BSS_START = .;
        *(.sbss .sbss.*)
        . = ALIGN(16);
        *(.bss .bss.*)
        _BSS_END_EXCL = .;
    } > ram

    /* define stack for boot core */
    . = ALIGN(8);
    /* stack size -- 16 pages for now */
    . = . + 16 * 4096;
    /* symbol for use in entry code */
    _STACK_BOTTOM = .;

    /* define symbols for use in kernel memory management code */
    _HEAP_START = _STACK_BOTTOM;
    _MEM_END = ORIGIN(ram) + LENGTH(ram);
}
