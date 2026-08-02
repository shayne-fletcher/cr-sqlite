#![no_std]

use core::ffi::{c_char, c_int, c_void};
use sqlite_nostd::{api_routines, sqlite3};

type BundleInit = extern "C" fn(*mut sqlite3, *mut *mut c_char, *mut api_routines) -> *mut c_void;

// Keep the Rust bundle in the final cdylib. The C initializer calls this
// symbol after SQLite supplies its extension API table.
#[used]
static CRSQL_BUNDLE_INIT: BundleInit = crsql_bundle::sqlite3_crsqlrustbundle_init;

extern "C" {
    fn crsqlite_c_init(
        db: *mut sqlite3,
        error: *mut *mut c_char,
        api: *const api_routines,
    ) -> c_int;
}

/// SQLite's standard loadable-extension entrypoint.
#[no_mangle]
pub unsafe extern "C" fn sqlite3_crsqlite_init(
    db: *mut sqlite3,
    error: *mut *mut c_char,
    api: *const api_routines,
) -> c_int {
    unsafe { crsqlite_c_init(db, error, api) }
}
