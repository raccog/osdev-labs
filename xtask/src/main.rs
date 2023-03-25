use xshell::{cmd, Shell};

mod flags {
    use anyhow::bail;
    use std::{vec, vec::Vec};
    use xflags;
    use xshell::{cmd, Shell};

    xflags::xflags! {
        src "./src/main.rs"

        cmd xtask {
            cmd check {
                required --target target: String
                optional --json-message-format
            }
        }
    }

    const ALL_TARGETS: [&'static str; 2] = ["aarch64-unknown-none", "x86_64-unknown-uefi"];

    #[derive(Debug)]
    pub struct Xtask {
        pub subcommand: XtaskCmd,
    }

    #[derive(Debug)]
    pub enum XtaskCmd {
        Check(Check),
    }

    #[derive(Debug)]
    pub struct Check {
        pub target: String,
        pub json_message_format: bool,
    }

    impl Check {
        pub fn run(self, sh: &Shell) -> anyhow::Result<()> {
            let targets = if self.target == "all" {
                Vec::from(ALL_TARGETS)
            } else {
                vec![self.target.as_str()]
            };

            for target in targets {
                let binary = match target {
                    "aarch64-unknown-none" => "aarch64-qemu",
                    "x86_64-unknown-uefi" => "x86_64-uefi",
                    _ => bail!("Invalid target: {}", target),
                };
                let mut flags = vec![
                    "-Zbuild-std=core,compiler_builtins,alloc",
                    "-Zbuild-std-features=compiler-builtins-mem",
                ];
                if self.json_message_format {
                    flags.push("--message-format=json");
                }

                cmd!(sh, "cargo check -p {binary} --target {target} {flags...}").run()?;
            }

            Ok(())
        }
    }

    #[allow(dead_code)]
    impl Xtask {
        pub fn from_env_or_exit() -> Self {
            Self::from_env_or_exit_()
        }
        pub fn from_env() -> xflags::Result<Self> {
            Self::from_env_()
        }
        pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
            Self::from_vec_(args)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let flags = flags::Xtask::from_env_or_exit();

    let sh = Shell::new()?;
    cmd!(sh, "pwd").run()?;

    match flags.subcommand {
        flags::XtaskCmd::Check(check) => check.run(&sh)?,
    }

    Ok(())
}
