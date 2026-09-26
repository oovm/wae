use crate::error::{BuildError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformSpec {
    pub id: &'static str,
    pub package_dir: &'static str,
    pub lib_file: &'static str,
    pub triple: &'static str,
}

const PLATFORMS: &[PlatformSpec] = &[
    PlatformSpec {
        id: "win32-x64",
        package_dir: "wae-win32-x64",
        lib_file: "win32-x64-msvc.node",
        triple: "x86_64-pc-windows-msvc",
    },
    PlatformSpec {
        id: "win32-arm64",
        package_dir: "wae-win32-arm64",
        lib_file: "win32-arm64-msvc.node",
        triple: "aarch64-pc-windows-msvc",
    },
    PlatformSpec {
        id: "darwin-x64",
        package_dir: "wae-darwin-x64",
        lib_file: "darwin-x64.node",
        triple: "x86_64-apple-darwin",
    },
    PlatformSpec {
        id: "darwin-arm64",
        package_dir: "wae-darwin-arm64",
        lib_file: "darwin-arm64.node",
        triple: "aarch64-apple-darwin",
    },
    PlatformSpec {
        id: "linux-x64",
        package_dir: "wae-linux-x64",
        lib_file: "linux-x64-gnu.node",
        triple: "x86_64-unknown-linux-gnu",
    },
    PlatformSpec {
        id: "linux-arm64",
        package_dir: "wae-linux-arm64",
        lib_file: "linux-arm64-gnu.node",
        triple: "aarch64-unknown-linux-gnu",
    },
    PlatformSpec {
        id: "android-arm64",
        package_dir: "wae-android-arm64",
        lib_file: "android-arm64.node",
        triple: "aarch64-linux-android",
    },
    PlatformSpec {
        id: "ios-arm64",
        package_dir: "wae-ios-arm64",
        lib_file: "ios-arm64.node",
        triple: "aarch64-apple-ios",
    },
];

pub fn platform_by_id(id: &str) -> Result<&'static PlatformSpec> {
    PLATFORMS
        .iter()
        .find(|p| p.id == id)
        .ok_or_else(|| BuildError::UnknownPlatform(id.to_string()))
}

pub fn platform_by_triple(triple: &str) -> Result<&'static PlatformSpec> {
    PLATFORMS
        .iter()
        .find(|p| p.triple == triple)
        .ok_or_else(|| BuildError::UnknownPlatform(triple.to_string()))
}

pub fn host_platform() -> Result<&'static PlatformSpec> {
    platform_by_triple(wae_updater::host_triple())
}
