use embed_manifest::{embed_manifest, manifest::*, new_manifest};
use winres::{
    WindowsResource,
    VersionInfo,
};

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_manifest(
            new_manifest("dchuuninstall")
                // .requested_execution_level(ExecutionLevel::RequireAdministrator)
                .dpi_awareness(DpiAwareness::PerMonitorV2)
        )
        .expect("unable to embed manifest file");
    }
    println!("cargo:rerun-if-changed=build.rs");
    // Get version from Cargo.toml
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let version_parts: Vec<&str> = version.split('.').collect();
    let major: u64 = version_parts[0].parse().unwrap_or(0);
    let minor: u64 = version_parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch: u64 = version_parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);

    // Configure Windows resource
    let mut res = WindowsResource::new();
    
    // Set version information
    res.set_version_info(VersionInfo::PRODUCTVERSION, 
        (major << 48) | (minor << 32) | (patch << 16) | 0);
    res.set_version_info(VersionInfo::FILEVERSION, 
        (major << 48) | (minor << 32) | (patch << 16) | 0);
    
    // Set version strings and file info
    res.set("FileVersion", &format!("{}.{}.{}.0", major, minor, patch));
    res.set("ProductVersion", &format!("{}.{}.{}.0", major, minor, patch));
    res.set("FileDescription", &std::env::var("CARGO_PKG_DESCRIPTION").unwrap_or_default());
    res.set("ProductName", &std::env::var("CARGO_PKG_NAME").unwrap_or_default());
    // res.set("CompanyName", "Your Company Name");
    res.set("LegalCopyright", "© 2024 Kin|Jiaching. All rights reserved.");
    res.set("OriginalFilename", &format!("{}.exe", std::env::var("CARGO_PKG_NAME").unwrap()));
    res.set("InternalName", &std::env::var("CARGO_PKG_NAME").unwrap_or_default());
    
    // Compile the resources
    res.compile().unwrap();
}