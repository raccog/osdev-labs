# Milestone #000 - Dev Environment

This milestone is to create a cross-compilation developer environment that is easy to use for all architectures.

A normal binary crate called `xtask` is used as a wrapper around Cargo. This both makes the cross-compilation build steps easier and allows for any non-Cargo commands to be intertwined during the build process.

- [x] Static analysis for aarch64
- [x] Static analysis for all targets (including xtask itself)
- [x] Building for aarch64
- [x] Building for all targets
- [x] Package for x86_64-uefi (create partitioned disk image)
- [x] Running aarch64 on qemu
- [x] Running x86_64-uefi on qemu
- [ ] Find OVMF firmware or build it
- [ ] Clean build directories
- [ ] Fixup all TODOs

