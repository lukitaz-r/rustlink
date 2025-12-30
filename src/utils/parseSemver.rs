use once_cell::sync::Lazy;
use regex::Regex;

static SEMVER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"/^(?<major>0|[1-9]\d*)\.(?<minor>0|[1-9]\d*)\.(?<patch>0|[1-9]\d*)(?:-(?<prerelease>[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+(?<build>[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$/").unwrap()
});

#[derive(Debug)]
pub struct Semver {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub prerelease: Vec<String>,
    pub build: Vec<String>,
}

pub fn parse_semver(version: &str) -> Option<Semver> {
    let caps = SEMVER_REGEX.captures(version)?;

    // Extracción segura usando nombres de grupo como en JS
    let major = caps.name("major")?.as_str().parse().unwrap_or(0);
    let minor = caps.name("minor")?.as_str().parse().unwrap_or(0);
    let patch = caps.name("patch")?.as_str().parse().unwrap_or(0);

    let prerelease = caps
        .name("prerelease")
        .map(|m| m.as_str().split('.').map(|s| s.to_string()).collect())
        .unwrap_or_default();

    let build = caps
        .name("build")
        .map(|m| m.as_str().split('.').map(|s| s.to_string()).collect())
        .unwrap_or_default();
    Some(Semver {
        major,
        minor,
        patch,
        prerelease,
        build,
    })
}
