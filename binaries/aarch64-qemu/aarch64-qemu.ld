ENTRY(_entry_stage0)
SECTIONS {
    . = 0x40000000;
    _PROGRAM_START = .;
    .text.boot : { *(.text.boot) }
    .text : { *(.text*) }
    .data : { *(.data*) }
    .rodata : { *(.rodata*) }
    .bss : { *(.bss* COMMON) }

    _PROGRAM_END = .;
    _PROGRAM_SIZE = _PROGRAM_END - _PROGRAM_START;

    . = ALIGN(8);
    /* 16KB stack size */
    _STACK_SIZE = 0x4000;
    /* Stack starts from the bottom and ends here */
    _STACK_END = .;
    . += _STACK_SIZE;
    _STACK_START = .;
}
