// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

use serde_derive::{Deserialize, Serialize};
use url::Url;

/// The type or location of the license for the packaged software.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum LicenseType {
    /// When there are no License available at all.
    /// The item should in general never be used, but is provided for
    /// convenience.
    None,
    /// The remote location of an url, this can be used when there is no
    /// expression available for the package you want to create.
    /// Depending on the package created the license may get downloaded during
    /// updated and embedded in the package.
    Location(Url),
    /// Allows specifying an expression of the License Type to use for the
    /// package.
    ///
    /// ### Notes
    ///
    /// No validation is done on this expression, and it is your responsibility
    /// to ensure the expression is valid for the packages that you are
    /// creating.
    Expression(String),
    /// Allows specifying both the expression and the remote location of a
    /// license. The item is preferred to be used when targeting multiple
    /// package managers.
    ExpressionAndLocation {
        /// Allows specifying an expression of the License Type to use for the
        /// package.
        expression: String,
        /// The remote location of an url
        url: Url,
    },
}

impl Default for LicenseType {
    fn default() -> Self {
        Self::None
    }
}
