.globl _entry_stage0
.extern _STACK_START
.extern _entry_stage1

.section ".text.boot"

_entry_stage0:
    ldr x30, =_STACK_START
    mov sp, x30
    bl _entry_stage1
    b .
