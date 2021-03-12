// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

use semver::Version;
use serde_derive::{Deserialize, Serialize};
use url::Url;

use crate::package::{chocolatey, Validate};

/// The type or location of the license for the packaged software.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum LicenseType {
    /// When there are no License available at all.
    /// The item should in general never be used, but is provided for
    /// convenience.
    None,
    /// Allows specifying an expression of the License Type to use for the
    /// package.
    ///
    /// ### Notes
    ///
    /// No validation is done on this expression, and it is your responsibility
    /// to ensure the expression is valid for the packages that you are
    /// creating.
    Expression(String),
    /// The remote location of an url, this can be used when there is no
    /// expression available for the package you want to create.
    /// Depending on the package created the license may get downloaded during
    /// updated and embedded in the package.
    Location(Url),
    /// Allows specifying both the expression and the remote location of a
    /// license. The item is preferred to be used when targeting multiple
    /// package managers.
    ExpressionAndLocation {
        /// Allows specifying an expression of the License Type to use for the
        /// package.
        expression: String,
        /// The remote location of an url
        location: Url,
    },
}

/// Stores common values that are related to 1 or more package managers.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PackageMetadata {
    /// The identifier of the package.
    id: String,

    /// The version of the package, can be automatically updated and
    /// is not necessary to initally be set.
    #[serde(default = "default_version")]
    pub version: Version,

    /// The list of maintainers that are responsible for the creating and
    /// maintaining of the package(s).
    #[serde(default = "default_maintainer")]
    maintainers: Vec<String>,

    /// The main endpoint (homepage) of the software.
    pub project_url: Url,

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
    /// licese = { expression = "MIT", location = "https://some-page.org/license" }
    /// ```
    ///
    /// ### Notes
    ///
    /// If creating a chocolatey package, a license url is necessary when
    /// pushing to the chocolatey repository.
    pub license: LicenseType,

    /// The short description of the software that will be packaged.
    pub summary: String,

    /// The metadata that are only related to Chocolatey packages.
    pub chocolatey: Option<chocolatey::ChocolateyMetadata>,
}

fn default_version() -> Version {
    Version::parse("0.0.0").unwrap()
}

fn default_maintainer() -> Vec<String> {
    vec![match std::env::var("PKG_MAINTAINER") {
        Ok(maintainer) => maintainer,
        Err(_) => whoami::username(),
    }]
}

impl PackageMetadata {
    /// Creates a new empty package with the specified `id`.
    ///
    /// ### Examples
    ///
    /// ```
    /// use pkg_upd::package::PackageMetadata;
    ///
    /// let pkg = PackageMetadata::new("my-awesome-package");
    ///
    /// println!("{:?}", pkg);
    /// ```
    ///
    /// ### Notes
    ///
    /// This function will not create the necessary metadata to create a valid
    /// package.
    pub fn new(id: &str) -> PackageMetadata {
        PackageMetadata {
            id: id.into(),
            version: default_version(),
            project_url: Url::parse("https:/_Software_Location_REMOVE_OR_FILL_OUT_").unwrap(),
            maintainers: default_maintainer(),
            license: LicenseType::None,
            summary: String::new(),
            chocolatey: None,
        }
    }

    /// Allows chaining the creation of a package while enabling Chocolatey
    /// packaging at the same time.
    ///
    /// ### Examples
    ///
    /// ```
    /// use pkg_upd::package::chocolatey::ChocolateyMetadata;
    /// use pkg_upd::package::PackageMetadata;
    ///
    /// let pkg = PackageMetadata::new("pkg-name").with_chocolatey(ChocolateyMetadata::new());
    ///
    /// println!("{:?}", pkg);
    /// ```
    pub fn with_chocolatey(mut self, data: chocolatey::ChocolateyMetadata) -> Self {
        self.chocolatey = Some(data);
        self
    }

    /// The identifier of the package, ie what the package will be called when
    /// it has been created.
    ///
    /// ### Notes
    ///
    /// Some package managers have certain restrictions with what characters an
    /// identifier can contain. These will be overridden when a package is
    /// created, with an option in the specific package manager metadata to
    /// override this automatic process. *Like Chocolatey recommending a
    /// lowercase identifier and requires this on the community repository.
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Validate for PackageMetadata {
    /// Validates the minimum data that are required to create a package.
    fn validate_data(&self) -> Vec<String> {
        let mut errors = vec![];

        if self.id.trim().is_empty() {
            errors.push("A identifier is required and must be specified!".into());
        }

        if let Some(chocolatey) = &self.chocolatey {
            errors.extend(chocolatey.validate_data());
        }

        errors
    }
}

impl Default for PackageMetadata {
    fn default() -> PackageMetadata {
        PackageMetadata::new("")
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn get_package() -> PackageMetadata {
        PackageMetadata {
            id: "test-package".into(),
            version: default_version(),
            maintainers: default_maintainer(),
            project_url: Url::parse("https://_Software_Location_REMOVE_OR_FILL_OUT_").unwrap(),
            license: LicenseType::None,
            summary: String::new(),
            chocolatey: None,
        }
    }

    #[test]
    fn new_should_create_default_metadata_with_expected_values() {
        let expected = get_package();

        let actual = PackageMetadata::new("test-package");

        assert_eq!(actual, expected);
    }

    #[test]
    fn default_should_create_metadata_with_default_values() {
        let mut expected = get_package();
        expected.id = String::new();

        let actual = PackageMetadata::default();

        assert_eq!(actual, expected);
    }

    #[test]
    fn with_chocolatey_should_creates_with_expected_values() {
        let mut expected = get_package();
        expected.chocolatey = Some(chocolatey::ChocolateyMetadata::new());

        let actual = PackageMetadata::new("test-package")
            .with_chocolatey(chocolatey::ChocolateyMetadata::new());

        assert_eq!(actual, expected);
    }

    #[test]
    fn id_should_return_set_identifier() {
        const EXPECTED: &str = "my-awesome-test-package";

        let pkg = PackageMetadata::new(EXPECTED);

        assert_eq!(pkg.id(), EXPECTED);
    }

    #[rstest(id, case(""), case("   "))]
    fn validate_should_show_error_on_empty_identifier(id: &str) {
        let pkg = PackageMetadata::new(id);

        let result = pkg.validate_data();

        assert_eq!(result, ["A identifier is required and must be specified!"]);
    }

    #[test]
    fn validate_should_include_chocolatey_validations() {
        let pkg = PackageMetadata::new("").with_chocolatey(chocolatey::ChocolateyMetadata::new());

        let result = pkg.validate_data();

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn validatie_should_not_create_messages_on_valid_data() {
        let pkg = PackageMetadata::new("some-id");

        let result = pkg.validate_data();

        assert_eq!(result.len(), 0);
    }
}
