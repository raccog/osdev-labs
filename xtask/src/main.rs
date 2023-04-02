use xshell::Shell;

mod flags {
    use anyhow::bail;
    use std::{str::FromStr, vec, vec::Vec};
    use xflags;
    use xshell::{cmd, Shell};

    // This macro defines all the command line arguments that can be used.
    xflags::xflags! {
        src "./src/main.rs"

        cmd xtask {
            cmd check {
                optional binary: Binary
                optional --json-message-format
            }
            cmd build {
                optional binary: Binary
                optional --json-message-format
            }
            cmd package {
                optional package_type: PackageType
            }
            cmd clean {}
            cmd run {
                optional package_type: PackageType
            }
        }
    }

    // A list of all the valid binaries
    const ALL_BINARIES: &'static [Binary] =
        &[Binary::Aarch64Qemu, Binary::X86_64Uefi, Binary::Xtask];

    // Some flags
    const JSON_MESSAGE_FORMAT_FLAG: &'static str = "--message-format=json";
    const CARGO_NO_STD_FLAGS: &'static [&'static str] = &[
        "-Zbuild-std=core,compiler_builtins,alloc",
        "-Zbuild-std-features=compiler-builtins-mem",
    ];

    fn get_binaries(binary: &Option<Binary>) -> Vec<Binary> {
        match binary {
            Some(binary) => vec![*binary],
            None => Vec::from(ALL_BINARIES),
        }
    }

    // All possible binary targets that can be built
    #[derive(Copy, Clone, Debug)]
    pub enum Binary {
        Aarch64Qemu,
        X86_64Uefi,
        Xtask,
    }

    impl FromStr for Binary {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "aarch64-qemu" => Ok(Self::Aarch64Qemu),
                "x86_64-uefi" => Ok(Self::X86_64Uefi),
                "xtask" => Ok(Self::Xtask),
                _ => Err("Invalid binary"),
            }
        }
    }

    impl Binary {
        /// Returns the name of this binary.
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Aarch64Qemu => "aarch64-qemu",
                Self::X86_64Uefi => "x86_64-uefi",
                Self::Xtask => "xtask",
            }
        }

        /// Returns true if this binary does not use the std library.
        pub fn is_no_std(&self) -> bool {
            match self {
                Self::Xtask => false,
                _ => true,
            }
        }

        /// Returns the required platform target to build for or an error if it could be built for any target.
        pub fn target(&self) -> anyhow::Result<&'static str> {
            match self {
                Self::Aarch64Qemu => Ok("aarch64-unknown-none"),
                Self::X86_64Uefi => Ok("x86_64-unknown-uefi"),
                _ => bail!("This binary does not need a specific target"),
            }
        }
    }

    /// All pre-defined packaging types.
    #[derive(Debug)]
    pub enum PackageType {
        Aarch64Qemu,
        X86_64Uefi,
    }

    impl FromStr for PackageType {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "aarch64-qemu" => Ok(Self::Aarch64Qemu),
                "x86_64-uefi" => Ok(Self::X86_64Uefi),
                _ => Err("Invalid package type"),
            }
        }
    }

    #[derive(Debug)]
    pub struct Xtask {
        pub subcommand: XtaskCmd,
    }

    impl Xtask {
        pub fn run(self, sh: &Shell) -> anyhow::Result<()> {
            self.subcommand.run(sh, &self)
        }
    }

    #[derive(Debug)]
    pub enum XtaskCmd {
        Check(Check),
        Build(Build),
        Package(Package),
        Clean(Clean),
        Run(Run),
    }

    impl XtaskCmd {
        pub fn run(&self, sh: &Shell, xtask: &Xtask) -> anyhow::Result<()> {
            match self {
                Self::Check(check) => check.run(sh, xtask),
                Self::Build(build) => build.run(sh, xtask),
                _ => bail!("Subcommand not implemented"),
            }
        }
    }

    #[derive(Debug)]
    pub struct Build {
        pub binary: Option<Binary>,
        pub json_message_format: bool,
    }

    #[derive(Debug)]
    pub struct Check {
        pub binary: Option<Binary>,
        pub json_message_format: bool,
    }

    #[derive(Debug)]
    pub struct Package {
        pub package_type: Option<PackageType>,
    }

    #[derive(Debug)]
    pub struct Clean {}

    #[derive(Debug)]
    pub struct Run {
        pub package_type: Option<PackageType>,
    }

    pub trait Subcommand {
        fn run(&self, sh: &Shell, xtask: &Xtask) -> anyhow::Result<()>;
    }

    /// Run a cargo subcommand
    fn cargo_run(
        subcommand: &str,
        binary: &Option<Binary>,
        sh: &Shell,
        _xtask: &Xtask,
        json_message_format: bool,
    ) -> anyhow::Result<()> {
        // Run for all binaries if `binary` is none
        let binaries = get_binaries(binary);

        for binary in binaries {
            // Add no_std flags if needed
            let mut flags = if binary.is_no_std() {
                let mut flags = Vec::from(CARGO_NO_STD_FLAGS);

                // Add specified target flag
                flags.push("--target");
                flags.push(binary.target()?);

                flags
            } else {
                Vec::new()
            };

            // JSON message format is needed for rust analyzer
            if json_message_format {
                flags.push(JSON_MESSAGE_FORMAT_FLAG);
            }

            let binary_str = binary.as_str();
            cmd!(sh, "cargo {subcommand} -p {binary_str} {flags...}").run()?;
        }

        Ok(())
    }

    impl Subcommand for Build {
        fn run(&self, sh: &Shell, xtask: &Xtask) -> anyhow::Result<()> {
            cargo_run("build", &self.binary, sh, xtask, self.json_message_format)
        }
    }

    impl Subcommand for Check {
        fn run(&self, sh: &Shell, xtask: &Xtask) -> anyhow::Result<()> {
            cargo_run("check", &self.binary, sh, xtask, self.json_message_format)
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
    let xtask = flags::Xtask::from_env_or_exit();

    let sh = Shell::new()?;

    xtask.run(&sh)?;

    Ok(())
}
