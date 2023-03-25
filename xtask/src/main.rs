use xshell::{cmd, Shell};

mod flags {
    use std::{vec, vec::Vec};
    use xflags;
    use xshell::{cmd, Shell};

    xflags::xflags! {
        src "./src/main.rs"

        cmd xtask {
            required --binary binary: String
            optional --json-message-format

            cmd check {}
            cmd build {}
        }
    }

    // A list of all the valid binaries
    const ALL_BINARIES: [&'static str; 2] = ["aarch64-qemu", "x86_64-uefi"];

    fn run_cargo(sh: &Shell, subcommand: &str, binary: &str, flags: &[&str]) -> anyhow::Result<()> {
        cmd!(
            sh,
            "cargo {subcommand} -p {binary} --target binaries/{binary}/{binary}.json {flags...}"
        )
        .run()?;

        Ok(())
    }

    #[derive(Debug)]
    pub struct Xtask {
        pub subcommand: XtaskCmd,
        pub binary: String,
        pub json_message_format: bool,
    }

    impl Xtask {
        fn create_binary_list(&self) -> Vec<&str> {
            if self.binary == "all" {
                Vec::from(ALL_BINARIES)
            } else {
                vec![self.binary.as_str()]
            }
        }

        fn create_flags(&self) -> Vec<&'static str> {
            let mut flags = vec![
                "-Zbuild-std=core,compiler_builtins,alloc",
                "-Zbuild-std-features=compiler-builtins-mem",
            ];
            if self.json_message_format {
                flags.push("--message-format=json");
            }

            flags
        }

        pub fn run(self, sh: &Shell) -> anyhow::Result<()> {
            let binaries = self.create_binary_list();
            let flags = self.create_flags();
            for binary in binaries {
                run_cargo(sh, self.subcommand.as_str(), binary, flags.as_slice())?;
            }

            Ok(())
        }
    }

    #[derive(Debug)]
    pub enum XtaskCmd {
        Check(Check),
        Build(Build),
    }

    impl XtaskCmd {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Check(_) => "check",
                Self::Build(_) => "build",
            }
        }
    }

    #[derive(Debug)]
    pub struct Check {}

    #[derive(Debug)]
    pub struct Build {}

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
    let xtask = flags::Xtask::from_env_or_exit();

    let sh = Shell::new()?;
    cmd!(sh, "pwd").run()?;

    xtask.run(&sh)?;

    Ok(())
}
