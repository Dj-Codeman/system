// #![feature(try_trait_v2)]
#![cfg_attr(rust_comp_feature = "try_trait_v2", feature(try_trait_v2))]
pub mod errors;
#[deprecated(since = "0.1.0", note = "please use `custom_error` instead")]
pub mod errors_dep;
pub mod functions;
pub mod types;
