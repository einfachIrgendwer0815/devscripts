//! Conveniently automate tasks using shell scripts.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::derive_partial_eq_without_eq)]
#![warn(clippy::use_self)]
#![warn(clippy::too_many_lines)]
// `allow_attributes_without_reason` requires the feature `lint_reason`.
// Activating this lint should be reconsidered once `lint_reason` is stabilized.
// #![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::use_debug)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![cfg_attr(not(debug_assertions), deny(clippy::todo))]
#![cfg_attr(not(debug_assertions), deny(clippy::unimplemented))]
#![cfg_attr(feature = "docsrs", feature(doc_auto_cfg))]

pub mod config;
mod path;
