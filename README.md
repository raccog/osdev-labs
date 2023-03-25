# OS Dev Labs

## Workflow

I'm currently figuring out how I want the workflow to be structured. This includes all the commands for building, testing, static analysis, etc. Right now, I'm testing out the [xtask crate](https://github.com/matklad/cargo-xtask).

Here's a list of the currently working commands:

* `cargo xtask check --target [all|aarch64-unknown-none|x86_64-unknown-uefi]`

Currently, only static analysis is implemented, no building or running yet.

### Workflow Roadmap

[x] Static analysis for aarch64
[x] Static analysis for all targets (including xtask itself)
[ ] Building for aarch64
[ ] Building for all targets
[ ] Running aarch64 on qemu
[ ] Running x86_64-uefi on qemu
