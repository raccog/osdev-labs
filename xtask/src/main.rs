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
            optional --release

            cmd check {
                optional binary: Binary
                optional --json-message-format
            }
            cmd build {
                optional binary: Binary
                optional --json-message-format
            }
            cmd package {
                required package_type: PackageType
            }
            cmd clean {}
            cmd run {
                required package_type: PackageType
            }
        }
    }

    // A list of all the valid binaries
    const ALL_BINARIES: &'static [Binary] =
        &[Binary::Aarch64Qemu, Binary::RiscV64Qemu, Binary::X86_64Uefi];

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
        RiscV64Qemu,
        X86_64Uefi,
        Xtask,
    }

    impl FromStr for Binary {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "aarch64-qemu" => Ok(Self::Aarch64Qemu),
                "x86_64-uefi" => Ok(Self::X86_64Uefi),
                "riscv64-qemu" => Ok(Self::RiscV64Qemu),
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
                Self::RiscV64Qemu => "riscv64-qemu",
                Self::X86_64Uefi => "x86_64-uefi",
                Self::Xtask => "xtask",
            }
        }

        /// Returns the build directory for this target; either release or debug.
        pub fn build_directory(&self, release: bool) -> String {
            let binary = self.as_str();
            let build_type = if release { "release" } else { "debug" };
            format!("target/{}/{}", binary, build_type)
        }

        /// Returns true if this binary does not use the std library.
        pub fn is_no_std(&self) -> bool {
            match self {
                Self::Xtask => false,
                _ => true,
            }
        }

        /// Returns true if this binary needs to be built for a specific target.
        pub fn needs_specific_target(&self) -> bool {
            match self {
                Self::Xtask => false,
                _ => true,
            }
        }

        /// Returns the required platform target to build for or an error if it could be built for any target.
        pub fn target(&self) -> anyhow::Result<&'static str> {
            match self {
                Self::Aarch64Qemu => Ok("binaries/aarch64-qemu/aarch64-qemu.json"),
                Self::RiscV64Qemu => Ok("binaries/riscv64-qemu/riscv64-qemu.json"),
                Self::X86_64Uefi => Ok("binaries/x86_64-uefi/x86_64-uefi.json"),
                _ => bail!("This binary does not need a specific target"),
            }
        }
    }

    /// All pre-defined packaging types.
    #[derive(Copy, Clone, Debug)]
    pub enum PackageType {
        Aarch64Qemu,
        RiscV64Qemu,
        X86_64Uefi,
    }

    impl FromStr for PackageType {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "aarch64-qemu" => Ok(Self::Aarch64Qemu),
                "riscv64-qemu" => Ok(Self::RiscV64Qemu),
                "x86_64-uefi" => Ok(Self::X86_64Uefi),
                _ => Err("Invalid package type"),
            }
        }
    }

    impl PackageType {
        pub fn binary(&self) -> Binary {
            match self {
                Self::Aarch64Qemu => Binary::Aarch64Qemu,
                Self::RiscV64Qemu => Binary::RiscV64Qemu,
                Self::X86_64Uefi => Binary::X86_64Uefi,
            }
        }
    }

    #[derive(Debug)]
    pub struct Xtask {
        pub release: bool,
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
        pub fn as_trait(&self) -> anyhow::Result<&dyn Subcommand> {
            match self {
                Self::Check(check) => Ok(check),
                Self::Build(build) => Ok(build),
                Self::Package(package) => Ok(package),
                Self::Run(run) => Ok(run),
                _ => bail!("Subcommand not implemented"),
            }
        }

        pub fn run(&self, sh: &Shell, xtask: &Xtask) -> anyhow::Result<()> {
            let subcmd = self.as_trait()?;
            subcmd.run(sh, xtask)
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
        pub package_type: PackageType,
    }

    #[derive(Debug)]
    pub struct Clean {}

    #[derive(Debug)]
    pub struct Run {
        pub package_type: PackageType,
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
        let mut binaries = get_binaries(binary);

        // Only add xtask binary if JSON message format is on.
        // This is becuase xtask is automatically checked and only needs to be checked again if its JSON message
        // is needed as output.
        if json_message_format {
            binaries.push(Binary::Xtask);
        }

        for binary in binaries {
            // Add no_std flags if needed
            let mut flags = if binary.is_no_std() {
                Vec::from(CARGO_NO_STD_FLAGS)
            } else {
                Vec::new()
            };

            // Add specified target flag if necessary
            if binary.needs_specific_target() {
                flags.push("--target");
                flags.push(binary.target()?);
            }

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

    impl Subcommand for Package {
        fn run(&self, sh: &Shell, xtask: &Xtask) -> anyhow::Result<()> {
            let binary = self.package_type.binary();

            // Build the needed binary before packaging the distribution.
            let build = Build {
                binary: Some(binary),
                json_message_format: false,
            };
            build.run(sh, xtask)?;

            match self.package_type {
                PackageType::Aarch64Qemu | PackageType::RiscV64Qemu => {
                    // This binary does not need any packaging
                }
                PackageType::X86_64Uefi => {
                    let build_dir = binary.build_directory(xtask.release);
                    // EFI System Partition path
                    let esp_path = format!("{}/esp.img", build_dir);
                    let disk_path = format!("{}/disk.img", build_dir);
                    let binary_path = format!("{}/{}.efi", build_dir, binary.as_str());

                    // TODO: Ensure all these executables are available on the host system.
                    // Create 64MB EFI System Partition
                    cmd!(sh, "dd if=/dev/zero of={esp_path} bs=1M count=64").run()?;
                    // Format to FAT32
                    cmd!(sh, "mkfs.vfat -F 32 {esp_path}").run()?;
                    // Create directories and copy bootloader
                    cmd!(sh, "mmd -D s -i {esp_path} '::/EFI'").run()?;
                    cmd!(sh, "mmd -D s -i {esp_path} '::/EFI/BOOT'").run()?;
                    cmd!(
                        sh,
                        "mcopy -D o -i {esp_path} {binary_path} '::/EFI/BOOT/BOOTX64.EFI'"
                    )
                    .run()?;

                    // TODO: Use hdiutil on MacOS instead of parted
                    // Create 66MB disk image
                    cmd!(sh, "dd if=/dev/zero of={disk_path} bs=1M count=66").run()?;
                    // Create ESP partition
                    cmd!(sh, "parted -s {disk_path} mklabel gpt").run()?;
                    cmd!(sh, "parted -s {disk_path} mkpart ESP fat32 2048s 100%").run()?;
                    cmd!(sh, "parted -s {disk_path} set 1 esp on").run()?;

                    // Copy FAT32 file system to disk image
                    cmd!(
                        sh,
                        "dd if={esp_path} of={disk_path} bs=1M seek=1 count=64 conv=notrunc"
                    )
                    .run()?;
                }
            }

            Ok(())
        }
    }

    impl Subcommand for Run {
        fn run(&self, sh: &Shell, xtask: &Xtask) -> anyhow::Result<()> {
            // Package the needed distribution before running
            let package = Package {
                package_type: self.package_type,
            };
            package.run(sh, xtask)?;

            match self.package_type {
                PackageType::Aarch64Qemu => {
                    let build_dir = self.package_type.binary().build_directory(xtask.release);
                    let binary_path =
                        format!("{}/{}", build_dir, self.package_type.binary().as_str());
                    cmd!(
                        sh,
                        "qemu-system-aarch64 -machine virt -cpu cortex-a57 -kernel {binary_path} -nographic"
                    )
                    .run()?;
                }
                PackageType::RiscV64Qemu => {
                    let build_dir = self.package_type.binary().build_directory(xtask.release);
                    let binary_path =
                        format!("{}/{}", build_dir, self.package_type.binary().as_str());
                    cmd!(
                        sh,
                        "qemu-system-riscv64 -machine virt -kernel {binary_path} -nographic"
                    )
                    .run()?;
                }
                PackageType::X86_64Uefi => {
                    let build_dir = self.package_type.binary().build_directory(xtask.release);
                    // TODO: Automatically pull or build OVMF firmware
                    let ovmf_path = format!("{}/OVMF.fd", build_dir);
                    let disk_path = format!("{}/disk.img", build_dir);

                    let ovmf_drive = format!("file={},if=pflash,format=raw,readonly=on", ovmf_path);
                    let disk_drive = format!("file={},format=raw", disk_path);

                    cmd!(sh, "qemu-system-x86_64 -drive {ovmf_drive} -drive {disk_drive} -cpu qemu64 -net none -serial stdio").run()?;
                }
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
    let xtask = flags::Xtask::from_env_or_exit();

    let sh = Shell::new()?;

    xtask.run(&sh)?;

    Ok(())
}
