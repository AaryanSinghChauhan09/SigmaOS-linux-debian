// SPDX-License-Identifier: MIT
//! SigmaOS-linux-debian — Real Debian Packaging, Control File & DPKG Compatibility Core
//!
//! Principles:
//! - NO STUBS: Strict Debian Policy control file generation, dependency serialization, dpkg query syntax
//! - ARRANGE-ACT-ASSERT: Comprehensive unit test coverage

#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::fmt;

/// Debian package prioritization level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebPriority {
    Required,
    Important,
    Standard,
    Optional,
    Extra,
}

impl fmt::Display for DebPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Required => write!(f, "required"),
            Self::Important => write!(f, "important"),
            Self::Standard => write!(f, "standard"),
            Self::Optional => write!(f, "optional"),
            Self::Extra => write!(f, "extra"),
        }
    }
}

/// Debian archive component section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebSection {
    Admin,
    Comm,
    Database,
    Debug,
    Devel,
    Editors,
    Fonts,
    Games,
    Graphics,
    Libs,
    Net,
    Science,
    Shells,
    Sound,
    Utils,
    Video,
    Web,
}

impl fmt::Display for DebSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admin => write!(f, "admin"),
            Self::Comm => write!(f, "comm"),
            Self::Database => write!(f, "database"),
            Self::Debug => write!(f, "debug"),
            Self::Devel => write!(f, "devel"),
            Self::Editors => write!(f, "editors"),
            Self::Fonts => write!(f, "fonts"),
            Self::Games => write!(f, "games"),
            Self::Graphics => write!(f, "graphics"),
            Self::Libs => write!(f, "libs"),
            Self::Net => write!(f, "net"),
            Self::Science => write!(f, "science"),
            Self::Shells => write!(f, "shells"),
            Self::Sound => write!(f, "sound"),
            Self::Utils => write!(f, "utils"),
            Self::Video => write!(f, "video"),
            Self::Web => write!(f, "web"),
        }
    }
}

/// Debian debian/control binary package stanza.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebControl {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub priority: DebPriority,
    pub section: DebSection,
    pub depends: Vec<String>,
}

impl DebControl {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        package: &str,
        version: &str,
        architecture: &str,
        maintainer: &str,
        description: &str,
        priority: DebPriority,
        section: DebSection,
        depends: Vec<String>,
    ) -> Self {
        Self {
            package: package.to_string(),
            version: version.to_string(),
            architecture: architecture.to_string(),
            maintainer: maintainer.to_string(),
            description: description.to_string(),
            priority,
            section,
            depends,
        }
    }

    /// Format dependencies as comma-delimited Debian Depends field.
    pub fn depends_line(&self) -> String {
        self.depends.join(", ")
    }

    /// Render canonical debian/control file stanza.
    pub fn to_control_file(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Package: {}\n", self.package));
        out.push_str(&format!("Version: {}\n", self.version));
        out.push_str(&format!("Section: {}\n", self.section));
        out.push_str(&format!("Priority: {}\n", self.priority));
        out.push_str(&format!("Architecture: {}\n", self.architecture));
        if !self.depends.is_empty() {
            out.push_str(&format!("Depends: {}\n", self.depends_line()));
        }
        out.push_str(&format!("Maintainer: {}\n", self.maintainer));
        out.push_str(&format!("Description: {}\n", self.description));
        out
    }
}

/// DPKG command generator for Debian package state inspections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpkgQuery {
    pub package: String,
}

impl DpkgQuery {
    pub fn new(package: &str) -> Self {
        Self {
            package: package.to_string(),
        }
    }

    /// Command to inspect detailed package installation status.
    pub fn status_command(&self) -> String {
        format!("dpkg -s {}", self.package)
    }

    /// Command to list all installed packages on Debian substrate.
    pub fn list_command() -> String {
        "dpkg -l".to_string()
    }

    /// APT installation command for this package.
    pub fn install_command(&self) -> String {
        format!("apt-get install -y {}", self.package)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deb_priority_display() {
        // Arrange & Act & Assert
        assert_eq!(DebPriority::Required.to_string(), "required");
        assert_eq!(DebPriority::Optional.to_string(), "optional");
    }

    #[test]
    fn test_deb_section_display() {
        // Arrange & Act & Assert
        assert_eq!(DebSection::Devel.to_string(), "devel");
        assert_eq!(DebSection::Web.to_string(), "web");
    }

    #[test]
    fn test_depends_line() {
        // Arrange
        let ctrl = DebControl::new(
            "sigma-core",
            "1.0.0",
            "amd64",
            "SigmaOS Core Team <core@sigmaos.org>",
            "Core userspace libraries",
            DebPriority::Standard,
            DebSection::Libs,
            vec!["libc6 (>= 2.31)".into(), "libssl3".into()],
        );
        // Act
        let dep = ctrl.depends_line();
        // Assert
        assert_eq!(dep, "libc6 (>= 2.31), libssl3");
    }

    #[test]
    fn test_to_control_file() {
        // Arrange
        let ctrl = DebControl::new(
            "sigmaos-desktop",
            "0.1.0",
            "amd64",
            "SigmaOS Desktop Team <desktop@sigmaos.org>",
            "Omarchy-inspired desktop environment",
            DebPriority::Optional,
            DebSection::Utils,
            vec!["wayland".into()],
        );
        // Act
        let file_content = ctrl.to_control_file();
        // Assert
        assert!(file_content.contains("Package: sigmaos-desktop"));
        assert!(file_content.contains("Priority: optional"));
        assert!(file_content.contains("Section: utils"));
        assert!(file_content.contains("Depends: wayland"));
    }

    #[test]
    fn test_control_no_depends() {
        // Arrange
        let ctrl = DebControl::new(
            "meta-pkg",
            "0.0.1",
            "all",
            "SigmaOS <info@sigmaos.org>",
            "Pure metadata package",
            DebPriority::Optional,
            DebSection::Admin,
            Vec::new(),
        );
        // Act
        let file_content = ctrl.to_control_file();
        // Assert
        assert!(!file_content.contains("Depends:"));
    }

    #[test]
    fn test_dpkg_status_command() {
        // Arrange
        let q = DpkgQuery::new("systemd");
        // Act & Assert
        assert_eq!(q.status_command(), "dpkg -s systemd");
    }

    #[test]
    fn test_dpkg_install_command() {
        // Arrange
        let q = DpkgQuery::new("neovim");
        // Act & Assert
        assert_eq!(q.install_command(), "apt-get install -y neovim");
    }
}
