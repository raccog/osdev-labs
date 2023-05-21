.globl _entry_stage0
.extern _STACK_START
.extern _entry_stage1

.section ".text.boot"

_entry_stage0:
    la sp, _STACK_START
    j _entry_stage1
    j .