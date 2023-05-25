# OS Dev Labs

## Workflow

I'm currently figuring out how I want the workflow to be structured. This includes all the commands for building, testing, static analysis, etc. Right now, I'm testing out the [xtask crate](https://github.com/matklad/cargo-xtask).

Here's a list of the currently working commands:

* `cargo xtask check [BINARY] [--json-message-format]`
* `cargo xtask build [BINARY] [--json-message-format]`
* `cargo xtask package PACKAGE`
* `cargo xtask run PACKAGE`

`BINARY` can be one of the following options (or it can be left blank to run for all binaries):

* `aarch64-qemu`
* `x86_64-uefi`

`PACKAGE` needs to be one of the following options:

* `aarch64-qemu`
* `x86_64-uefi`

### Operating System Roadmap

Roadmaps are contained in the directory `docs/roadmaps`.
