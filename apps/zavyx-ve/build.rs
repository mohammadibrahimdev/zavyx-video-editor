//! Windows only: embed the app icon and version info (VERSIONINFO) into `zavyx-ve.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing (`winresource` is only a build-dependency on Windows
//! hosts). A missing resource compiler is a warning, unless `ZAVYX_REQUIRE_WINRES=1` (for
//! release builds) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/zavyx-ve.ico");
    println!("cargo:rerun-if-env-changed=ZAVYX_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    windows::embed_resources();
}

#[cfg(windows)]
mod windows {
    pub fn embed_resources() {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../assets/app-icon/zavyx-ve.ico")
            .set("ProductName", "Zavyx VE")
            .set("FileDescription", "Zavyx Video Editor")
            .set("LegalCopyright", "Copyright (c) Mohammad Ibrahim and the Zavyx VE contributors. MIT OR Apache-2.0.")
            .set("OriginalFilename", "zavyx-ve.exe")
            .set("InternalName", "zavyx-ve");
        if let Err(e) = res.compile() {
            if std::env::var_os("ZAVYX_REQUIRE_WINRES").is_some() {
                panic!("embedding Windows resources failed: {e}");
            }
            println!("cargo:warning=zavyx-ve.exe built without icon/version resources: {e}");
        }
    }
}

/// Cross-compiling for Windows from macOS or Linux: `winresource` is not available on this host.
#[cfg(not(windows))]
mod windows {
    pub fn embed_resources() {
        println!("cargo:warning=zavyx-ve.exe built without icon/version resources (cross-compiled from a non-Windows host)");
    }
}
