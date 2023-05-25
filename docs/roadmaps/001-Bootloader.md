# Milestone #001 - Bootloader

This milestone is to load and run my kernel from memory using a bootloader.

## Steps

- [ ] Design memory map struct
- [ ] Get memory map (dynamically or manually)
- [ ] Design physical page frame allocator
- [ ] Design simple malloc()
- [ ] X86 only
    - [ ] Replace GDT
    - [ ] Replace IDT
- [ ] Read kernel to memory
    - [ ] X86: read from file system using UEFI
    - [ ] arm/riscv(qemu): read from pre-loaded memory
- [ ] Parse kernel elf sections and load to specified spots
- [ ] Set up stack area
- [ ] Set up struct to send to kernel
- [ ] Fill out page directory mappings
- [ ] Turn on paging
- [ ] Jump into kernel entry point
