MEMORY { 
    /* qemu -kernel jumps here */
    ram  (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
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
    _STACK_PTR = .;
}
