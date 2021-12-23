//! Trace and return on-disk locations of all file dependencies from a source file.
#![forbid(unsafe_code)]

mod trace_file;

#[cfg(test)]
mod tests;

/// Error macros
#[macro_use]
extern crate anyhow;
