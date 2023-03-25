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
                required --binary binary: String
                optional --json-message-format
            }
        }
    }

    // A list of all the valid binaries
    const ALL_BINARIES: [&'static str; 2] = ["aarch64-qemu", "x86_64-uefi"];

    fn create_binary_list(binary: &str) -> Vec<&str> {
        if binary == "all" {
            Vec::from(ALL_BINARIES)
        } else {
            vec![binary]
        }
    }

    fn create_flags(json_message_format: bool) -> Vec<&'static str> {
        let mut flags = vec![
            "-Zbuild-std=core,compiler_builtins,alloc",
            "-Zbuild-std-features=compiler-builtins-mem",
        ];
        if json_message_format {
            flags.push("--message-format=json");
        }

        flags
    }

    fn run_cargo(sh: &Shell, subcommand: &str, binary: &str, flags: &[&str]) -> anyhow::Result<()> {
        let target = match binary {
            "aarch64-qemu" => "aarch64-unknown-none",
            "x86_64-uefi" => "x86_64-unknown-uefi",
            _ => bail!("Invalid binary: {}", binary),
        };

        cmd!(
            sh,
            "cargo {subcommand} -p {binary} --target {target} {flags...}"
        )
        .run()?;

        Ok(())
    }

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
        pub binary: String,
        pub json_message_format: bool,
    }

    impl Check {
        pub fn run(self, sh: &Shell) -> anyhow::Result<()> {
            let binaries = create_binary_list(self.binary.as_str());
            let flags = create_flags(self.json_message_format);
            for binary in binaries {
                run_cargo(sh, "check", binary, flags.as_slice())?;
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
