//! # A crate for GameBoy (Color) development.
//! Rust-GB is a both toolchain and library for compiling Rust code into
//! Nintendo GameBoy. It compiles Rust code into valid GameBoy ROM through
//! LLVM-CBE and GBDK-2020.
//!
//! ## Install the compiler
//! Install a Rust-GB compiler with `cargo install`
//! 
//! ``` bash
//! cargo install rust-gb --features=compiler
//! ```
//!
//! `compiler` feature is required when installing the Rust-GB compiler.
//! If not, binary will not be installed.
//!
//! > **Note:** Right now, you can't run the Rust-GB compiler other than Linux x64. This is because the
//! > GameBoy compilation process requires some "external binaries". We're going to solve this problem in
//! > the future by packaging Rust-GB compiler for a platform-specific package manager (`winget`, `homebrew`, `pacman` etc.)
//!
//! ## Setup a project
//! 
//! 
//! ## Compile your project
//! ## Execute your ROM
#![doc = document_features::document_features!()]
#![doc(html_logo_url = "https://github.com/zlfn/rust-gb/blob/main/media/ferris-gb.png?raw=true")]

#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(strict_provenance)]
#![feature(format_args_nl)]
#![feature(doc_cfg)]

pub mod io;

pub mod mmio;

pub mod drawing;

#[cfg(feature = "prototype")]
pub mod memory;

pub mod gbdk_c;

