// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

//! Contains data related to building packages.
//! This can be metadata for specific package managers, build scripts, urls and
//! version regexes and more.
//!
//! ### Examples
//!
//! The following is an example of a minimal configuration
//! file for a package in the `TOML` language.
//!
//! ```toml
//! [metadata]
//! id = "test-package"
//! version = "1.0.0"
//! project_url = "https://test.com/test-package"
//!
//! [metadata.chocolatey]
//! lowercase_id = true
//! authors = ["AdmiringWorm"]
//! description = """\
//!     This is a multiline description \
//!     example of what will be included \
//!     in a Chocolatey package. \
//!     The ending \\ means that all whitespace wil be trimmed \
//! """
//! ```

pub mod chocolatey;

/// Validates any item that implements this trait for missing information, or
/// any information that will cause a failure.
pub trait Validate {
	fn validate_data(&self) -> Vec<String>;
}
