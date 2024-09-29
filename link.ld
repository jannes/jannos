MEMORY { 
    /* qemu uses OPENSBI which jumps to address specified as ORIGIN */
    /* LENGTH is a required parameter here, but actually it is dynamic
       so just set it to a dummy value that is large enough to fit static sections */
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

    /* Define a region used as temporary stack for boot core */
    /* SP needs to be 16 byte aligned */
    . = ALIGN(16);
    /* Use large enough stack size to not overflow in init code
       16 pages should suffice */
    . = . + 16 * 4096;
    /* Symbol for use in entry code */
    _TMP_BOOT_STACK_BOTTOM = .;

    /* Export heap start to use in memory initialization code */
    /* TODO: heap should actually start right after bss section,
    * so heap setup should occur after boot core switches away from 
    * temporary stack */
    _HEAP_START = .;


/* In initialization code, after determining the amount of memory
   stacks for each boot core should be set up at the very top of
   the memory space. With mem_end being the exclusive end address,
   each core's SP is determined by:

   mem_end - (hart_id * STACK_SIZE) */
}
