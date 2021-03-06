#![allow(clippy::all)]

//! D-Bus interface implementations.
//!
//! All submodules are generated by `dbus-codegen` and not part of the public APIs. However, since this crate is already
//! private, making everything here public is only to include them in the documentation for development convenience.

/// `org.freedesktop.IBus.Portal`
pub mod ibus;
