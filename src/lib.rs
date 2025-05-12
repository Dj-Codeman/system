pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub mod core;
#[cfg(unix)]
pub mod platform;

// pub mod errors;

// #[deprecated(since = "0.1.0", note = "please use `errors` instead")]
// pub mod errors_dep;
// pub mod functions;
// pub mod logger;
// pub mod types;
// pub mod version;

#[path = "tests/errors.rs"]
pub mod errors_test;
#[path = "tests/functions.rs"]
pub mod function_test;
#[path = "tests/rb.rs"]
pub mod rb_test;
#[path = "tests/rwarc.rs"]
pub mod rwarc_test;
#[path = "tests/stringy.rs"]
pub mod stringy_test;
#[path = "tests/pathtype.rs"]
pub mod types_test;
#[path = "tests/version.rs"]
pub mod version_test;
