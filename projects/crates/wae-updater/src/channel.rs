use crate::error::{Result, UpdateError};

/// Release channel used to resolve which GitHub Release to compare against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseChannel {
    /// Latest non-prerelease, non-draft release.
    Stable,
    /// Latest release including prereleases.
    Beta,
    /// Explicit tag (nightly builds, hotfix pins, etc.).
    Pinned(String),
}

impl ReleaseChannel {
    pub fn parse(raw: &str) -> Result<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "" | "stable" => Ok(ReleaseChannel::Stable),
            "beta" | "prerelease" | "preview" => Ok(ReleaseChannel::Beta),
            other => Ok(ReleaseChannel::Pinned(other.to_string())),
        }
    }

    pub fn from_legacy(allow_prerelease: bool, tag: Option<&str>) -> Self {
        if let Some(tag) = tag.filter(|t| !t.is_empty()) {
            return ReleaseChannel::Pinned(tag.to_string());
        }
        if allow_prerelease {
            ReleaseChannel::Beta
        } else {
            ReleaseChannel::Stable
        }
    }

    pub fn allows_prerelease(&self) -> bool {
        matches!(self, ReleaseChannel::Beta | ReleaseChannel::Pinned(_))
    }

    pub fn pinned_tag(&self) -> Option<&str> {
        match self {
            ReleaseChannel::Pinned(tag) => Some(tag.as_str()),
            _ => None,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            ReleaseChannel::Stable => "stable",
            ReleaseChannel::Beta => "beta",
            ReleaseChannel::Pinned(tag) => tag,
        }
    }
}

/// Whether the updater should stop after version check or continue downloading/applying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadPolicy {
    /// Version check only — explicit / UI-driven flow.
    CheckOnly,
    /// Download when newer (silent background fetch, no apply).
    DownloadIfAvailable,
    /// Check, download, and replace the installed native artifact.
    DownloadAndApply,
}

impl DownloadPolicy {
    pub fn parse(raw: &str) -> Result<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "" | "checkonly" | "check-only" | "explicit" => Ok(DownloadPolicy::CheckOnly),
            "downloadifavailable" | "download-if-available" | "silent" | "background" => {
                Ok(DownloadPolicy::DownloadIfAvailable)
            }
            "downloadandapply" | "download-and-apply" | "auto" | "full" => {
                Ok(DownloadPolicy::DownloadAndApply)
            }
            other => Err(UpdateError::InvalidDownloadPolicy(other.to_string())),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            DownloadPolicy::CheckOnly => "checkOnly",
            DownloadPolicy::DownloadIfAvailable => "downloadIfAvailable",
            DownloadPolicy::DownloadAndApply => "downloadAndApply",
        }
    }
}
