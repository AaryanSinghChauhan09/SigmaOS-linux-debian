#[derive(Debug, Clone, PartialEq)]
pub struct DebianPackageControl {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub depends: Vec<String>,
}

pub struct DpkgParser;

impl DpkgParser {
    pub fn parse_control(raw: &str) -> Option<DebianPackageControl> {
        let mut pkg = None;
        let mut ver = None;
        let mut arch = None;
        let mut maint = None;
        let mut deps = Vec::new();

        for line in raw.lines() {
            if let Some((k, v)) = line.split_once(':') {
                let key = k.trim().to_lowercase();
                let val = v.trim();
                match key.as_str() {
                    "package" => pkg = Some(val.to_string()),
                    "version" => ver = Some(val.to_string()),
                    "architecture" => arch = Some(val.to_string()),
                    "maintainer" => maint = Some(val.to_string()),
                    "depends" => {
                        deps = val.split(',').map(|s| s.trim().to_string()).collect();
                    }
                    _ => {}
                }
            }
        }

        Some(DebianPackageControl {
            package: pkg?,
            version: ver?,
            architecture: arch.unwrap_or_else(|| "amd64".to_string()),
            maintainer: maint.unwrap_or_default(),
            depends: deps,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpkg_parser() {
        // Arrange
        let control = "Package: libc6\nVersion: 2.38-1\nArchitecture: amd64\nDepends: libgcc-s1\n";
        // Act
        let deb = DpkgParser::parse_control(control).unwrap();
        // Assert
        assert_eq!(deb.package, "libc6");
        assert_eq!(deb.depends, vec!["libgcc-s1"]);
    }
}
