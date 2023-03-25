# OS Dev Labs

## Workflow

I'm currently figuring out how I want the workflow to be structured. This includes all the commands for building, testing, static analysis, etc. Right now, I'm testing out the [xtask crate](https://github.com/matklad/cargo-xtask).

Here's a list of the currently working commands:

* `cargo xtask check --binary [all|aarch64-qemu|x86_64-uefi]`
* `cargo xtask build --binary [all|aarch64-qemu|x86_64-uefi]`

Currently, only static analysis and building are implemented, no running yet.

### Workflow Roadmap

[x] Static analysis for aarch64
[x] Static analysis for all targets (including xtask itself)
[x] Building for aarch64
[x] Building for all targets
[ ] Running aarch64 on qemu
[ ] Running x86_64-uefi on qemu
