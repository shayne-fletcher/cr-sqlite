use std::env;
use std::path::{Path, PathBuf};

fn main() {
    if env::var("CARGO_CFG_TARGET_FAMILY").as_deref() == Ok("unix") {
        println!("cargo:rustc-link-lib=c");
    }

    let source_dir = PathBuf::from("core/src");
    let sqlite_dir = source_dir.join("sqlite");
    let sources = ["crsqlite.c", "changes-vtab.c", "ext-data.c"];
    let headers = [
        "changes-vtab.h",
        "consts.h",
        "crsqlite.h",
        "ext-data.h",
        "rust.h",
        "util.h",
    ];

    for source in sources {
        rerun_if_changed(&source_dir.join(source));
    }
    for header in headers {
        rerun_if_changed(&source_dir.join(header));
    }
    rerun_if_changed(&sqlite_dir.join("sqlite3.h"));
    rerun_if_changed(&sqlite_dir.join("sqlite3ext.h"));

    let mut build = cc::Build::new();
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos")
        && env::var_os("MACOSX_DEPLOYMENT_TARGET").is_none()
    {
        let minimum = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
            Ok("aarch64") => "11.0",
            _ => "10.12",
        };
        build.flag(format!("-mmacosx-version-min={minimum}"));
    }

    build
        .files(sources.map(|source| source_dir.join(source)))
        .include(&source_dir)
        .include(&sqlite_dir)
        .define("CRSQLITE_ENTRYPOINT", "crsqlite_c_init")
        .flag_if_supported("-std=c99")
        .pic(true)
        .warnings(true)
        .compile("crsqlite_c");
}

fn rerun_if_changed(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
}
