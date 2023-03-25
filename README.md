# OS Dev Labs

## Workflow

I'm currently figuring out how I want the workflow to be structured. This includes all the commands for building, testing, static analysis, etc. Right now, I'm testing out the [xtask crate](https://github.com/matklad/cargo-xtask).

Here's a list of the currently working commands:

* `cargo xtask check --target aarch64-unknown-none`

Yup, you can only do static analysis for aarch64 right now...

The plan is to add the following features to the workflow in order:

* Static analysis for all targets (including xtask itself)
* Building for aarch64
* Building for all targets
* Running aarch64 on qemu
* Running x86_64-uefi on qemu
