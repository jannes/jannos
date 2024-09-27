MEMORY { 
    /* qemu uses OPENSBI which jumps to this address */
    ram  (wxa) : ORIGIN = 0x80200000, LENGTH = 128M
}


SECTIONS {
    .text.boot : {
        *(.text.boot)
    } > ram

    .text : {
        *(.text)
    } > ram

    .data : {
        *(.data)
    } > ram

    .rodata : {
        *(.rodata)
    } > ram

    .bss : {
        *(.bss)
    } > ram

    . = ALIGN(8);
    /* stack size */
    . = . + 4096;
    /* symbol for use in entry code */
    _STACK_BOTTOM = .;

    /* define symbols for use in kernel memory management code */
    _HEAP_START = _STACK_BOTTOM;
    _MEM_END = ORIGIN(ram) + LENGTH(ram);
}
