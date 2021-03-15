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
//! summary = "Short summary of the software"
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
pub(crate) mod metadata;

pub use metadata::{LicenseType, PackageMetadata};
use serde_derive::{Deserialize, Serialize};

/// Validates any item that implements this trait for missing information, or
/// any information that will cause a failure.
pub trait Validate {
    fn validate_data(&self) -> Vec<String>;
}

/// Structure for holding all available data that a user can specify for a
/// package.
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
#[non_exhaustive]
pub struct PackageData {
    pub metadata: PackageMetadata,
}

impl PackageData {
    /// Creates a new instance of a structure holding user data.
    pub fn new(id: &str) -> PackageData {
        PackageData {
            metadata: PackageMetadata::new(id),
        }
    }
}

impl Validate for PackageData {
    /// Validates all stored information recursively
    fn validate_data(&self) -> Vec<String> {
        self.metadata.validate_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_data_with_default_values() {
        let expected = PackageData {
            metadata: PackageMetadata::new("test-package"),
        };

        let actual = PackageData::new("test-package");

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_should_create_validation_message() {
        let pkg = PackageData::new("");

        let result = pkg.validate_data();

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn validate_should_not_create_message_on_valid_data() {
        let pkg = PackageData::new("some-id");

        let result = pkg.validate_data();

        assert_eq!(result.len(), 0);
    }
}
