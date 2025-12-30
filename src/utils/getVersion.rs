use super::types::versions::Semver;
use serde::Serialize;

pub fn get_version_string() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn get_version_object() -> Semver {
    let version = env!("CARGO_PKG_VERSION");
    parse_semver(version)
}

fn parse_semver(v: &str) -> Semver {
    let parts: Vec<&str> = v.split('.').collect();
    let major = parts.get(0).unwrap_or(&"0").parse().unwrap_or(0);
    let minor = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);
    let patch_str = parts.get(2).unwrap_or(&"0");

    let (patch, pre) = if let Some(idx) = patch_str.find('-') {
        (
            patch_str[..idx].parse().unwrap_or(0),
            Some(patch_str[idx + 1..].to_string()),
        )
    } else {
        (patch_str.parse().unwrap_or(0), None)
    };
    Semver {
        major,
        minor,
        patch,
        pre,
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum VersionResult {
    String(&'static str),
    Object(Semver),
}

pub fn get_version(as_object: bool) -> VersionResult {
    if as_object {
        VersionResult::Object(get_version_object())
    } else {
        VersionResult::String(get_version_string())
    }
}

impl std::fmt::Display for Semver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre {
            write!(f, "-{}", pre)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for VersionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionResult::String(s) => write!(f, "{}", s),
            VersionResult::Object(v) => write!(f, "{}", v),
        }
    }
}
