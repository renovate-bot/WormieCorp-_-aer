// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

pub mod chocolatey;

use std::fmt::Display;
use std::path::PathBuf;

use pkg_license::LicenseType;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Description {
    None,
    Location {
        from: PathBuf,
        skip_start: u16,
        skip_end: u16,
    },
    Text(String),
}

impl PartialEq<str> for Description {
    fn eq(&self, right: &str) -> bool {
        self == &Description::Text(right.into())
    }
}

/// Stores common values that are related to 1 or more package managers.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[non_exhaustive]
pub struct PackageMetadata {
    /// The identifier of the package
    id: String,

    /// The list of maintainers that are responsible for the creating and
    /// maintaining of the package(s).
    #[serde(default = "crate::defaults::maintainer")]
    maintainers: Vec<String>,

    /// The main enpoints (homepage) of the software.
    pub summary: String,

    /// The main endpoint (homepage) of the software.
    project_url: Url,

    /// The type of the license, this can be either a supported expression (Like
    /// `MIT`, `GPL`, etc.) or an url the location of the license.
    ///
    /// ### Examples
    ///
    /// A `TOML` edition of only specifying a License Expression.
    /// ```toml
    /// [metadata]
    /// id = "test-package"
    /// project_url = "https://some-page.org"
    /// license = "MIT"
    /// ```
    ///
    /// A `TOML` edition of only specifying a License URL.
    /// ```toml
    /// [metadata]
    /// id = "test-package"
    /// project_url = "https://some-page.org"
    /// license = "https://some-page.org/license"
    /// ```
    ///
    /// A `TOML` edition of specifying both a License Expression and a License.
    /// This edition is recommended in most cases when creating packages for
    /// multiple package managers. URL.
    /// ```toml
    /// [metadata]
    /// id = "test-package"
    /// project_url = "https://some-page.org"
    /// license = { expression = "MIT", location = "https://some-page.org/license" }
    /// ```
    ///
    /// ### Notes
    ///
    /// If creating a chocolatey package, a license url is usually necessary
    /// when pushing to the chocolatey repository.
    #[serde(default)]
    license: LicenseType,

    chocolatey: Option<chocolatey::ChocolateyMetadata>,
}

impl PackageMetadata {
    /// Creates a new instance of the package metadata with the specified
    /// identifier.
    pub fn new(id: &str) -> PackageMetadata {
        PackageMetadata {
            id: id.to_owned(),
            maintainers: crate::defaults::maintainer(),
            summary: String::new(),
            project_url: Url::parse("https://example-repo.org").unwrap(),
            license: LicenseType::None,
            chocolatey: None,
        }
    }

    /// Returns the main identifier for the package.
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn chocolatey(&self) -> Option<&chocolatey::ChocolateyMetadata> {
        if let Some(ref choco) = self.chocolatey {
            Some(&choco)
        } else {
            None
        }
    }

    /// Returns the people responsible for creating and updating the package.
    pub fn maintainers(&self) -> &[String] {
        self.maintainers.as_slice()
    }

    /// Returns the url to the landing page of the software.
    pub fn project_url(&self) -> &Url {
        &self.project_url
    }

    pub fn license(&self) -> &LicenseType {
        &self.license
    }

    pub fn set_chocolatey(&mut self, choco: chocolatey::ChocolateyMetadata) {
        self.chocolatey = Some(choco);
    }

    pub fn set_maintainers<T>(&mut self, vals: &[T])
    where
        T: Display,
    {
        let mut maintainers = Vec::<String>::with_capacity(vals.len());

        for val in vals.iter() {
            maintainers.push(val.to_string());
        }

        self.maintainers = maintainers;
    }

    pub fn set_project_url(&mut self, url: &str) {
        let url = Url::parse(url).unwrap(); // We want a failure here to abort the program
        self.project_url = url;
    }

    pub fn set_license(&mut self, license: LicenseType) {
        self.license = license;
    }
}

impl Default for PackageMetadata {
    fn default() -> PackageMetadata {
        PackageMetadata::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_default_metadata_with_expected_values() {
        let expected = PackageMetadata {
            id: "test-package".to_owned(),
            maintainers: crate::defaults::maintainer(),
            project_url: Url::parse("https://example-repo.org").unwrap(),
            license: LicenseType::None,
            summary: String::new(),
            chocolatey: None,
        };

        let actual = PackageMetadata::new("test-package");

        assert_eq!(actual, expected);
    }

    #[test]
    fn default_should_create_default_metadata_with_expected_values() {
        let expected = PackageMetadata::new("");

        let actual = PackageMetadata::default();

        assert_eq!(actual, expected);
    }

    #[test]
    fn id_should_return_set_identifier() {
        const EXPECTED: &str = "my-awesome-test-package";

        let pkg = PackageMetadata::new(EXPECTED);

        assert_eq!(pkg.id(), EXPECTED);
    }

    #[test]
    fn maintainers_should_return_set_maintainers() {
        let expected = [
            "AdmiringWorm".to_owned(),
            "Some maintainer".to_owned(),
            "Some other".to_owned(),
        ];
        let mut pkg = PackageMetadata::new("test");
        pkg.maintainers = Vec::from(expected.clone());

        assert_eq!(pkg.maintainers(), expected);
    }

    #[test]
    fn project_url_should_return_set_project_url() {
        let expected = Url::parse("https://github.com/WormieCorp/pkg-upd").unwrap();
        let mut pkg = PackageMetadata::new("test");
        pkg.project_url = expected.clone();

        assert_eq!(pkg.project_url(), &expected);
    }
}
