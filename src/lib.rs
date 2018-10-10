#![crate_name = "voca_rs"]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]
pub mod case;
pub mod chop;
pub mod count;
pub mod index;
pub mod manipulate;
pub mod query;
pub mod split;
pub mod utils;

extern crate unicode_segmentation;
