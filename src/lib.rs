//! # native-sys
//!
//! Access undocumented Native API primitives and type definitions.
//!
//! This crate uses [`windows-sys`][windows-sys] underneath the hood rather than the
//! heavier, more "fully-featured" [`windows-rs`][windows-rs-crate] crate.
//!
//! ## Description
//!
//! - Access to undocumented Native API primitives and type definitions
//! - Uses the [`windows-sys`][windows-sys] crate for `no-std` environments and
//!   faster compilation
//! - Feature gates allow you to only use what you need
//! - Headers sourced from Process Hacker's NT headers
//!
//! [windows-rs]: https://crates.io/crates/windows
//! [windows-sys]: https://crates.io/crates/windows-sys

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(internal_features)]
#![warn(clippy::cargo)]
#![deny(clippy::perf)]
#![deny(clippy::all)]

#[cfg(not(any(all(windows, any(target_arch = "x86_64", target_arch = "x86")))))]
compile_error!(
    r"Current target is not supported.

    This crate supports the following targets:

- x86_64-pc-windows-msvc
- i686-pc-windows-msvc
"
);

pub mod bitfield;
#[cfg(feature = "ntbcd")]
pub mod ntbcd;
#[cfg(feature = "ntdbg")]
pub mod ntdbg;
#[cfg(feature = "ntdef")]
pub mod ntdef;
#[cfg(feature = "ntexapi")]
pub mod ntexapi;
#[cfg(feature = "ntgdi")]
pub mod ntgdi;
#[cfg(feature = "ntimage")]
pub mod ntimage;
#[cfg(feature = "ntioapi")]
pub mod ntioapi;
#[cfg(feature = "ntkeapi")]
pub mod ntkeapi;
#[cfg(feature = "ntldr")]
pub mod ntldr;
#[cfg(feature = "ntlpcapi")]
pub mod ntlpcapi;
#[cfg(feature = "ntmisc")]
pub mod ntmisc;
#[cfg(feature = "ntmmapi")]
pub mod ntmmapi;
#[cfg(feature = "ntnls")]
pub mod ntnls;
#[cfg(feature = "ntobapi")]
pub mod ntobapi;
#[cfg(feature = "ntpebteb")]
pub mod ntpebteb;
#[cfg(feature = "ntpfapi")]
pub mod ntpfapi;
#[cfg(feature = "ntpnpapi")]
pub mod ntpnpapi;
#[cfg(feature = "ntpoapi")]
pub mod ntpoapi;
#[cfg(feature = "ntpsapi")]
pub mod ntpsapi;
#[cfg(feature = "ntregapi")]
pub mod ntregapi;
#[cfg(feature = "ntrtl")]
pub mod ntrtl;
#[cfg(feature = "ntsam")]
pub mod ntsam;
#[cfg(feature = "ntseapi")]
pub mod ntseapi;
#[cfg(feature = "ntsmss")]
pub mod ntsmss;
#[cfg(feature = "ntsxs")]
pub mod ntsxs;
#[cfg(feature = "nttmapi")]
pub mod nttmapi;
#[cfg(feature = "nttp")]
pub mod nttp;
#[cfg(feature = "ntwow64")]
pub mod ntwow64;
#[cfg(feature = "ntxcapi")]
pub mod ntxcapi;
#[cfg(feature = "ntzwapi")]
pub mod ntzwapi;
#[cfg(feature = "subprocesstag")]
pub mod subprocesstag;
#[cfg(feature = "winntdef")]
pub mod winntdef;
#[cfg(feature = "winsta")]
pub mod winsta;
