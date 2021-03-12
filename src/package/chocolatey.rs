// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

//! Contains all data that can be used that are specific to chocolatey packages.
//! Variables that are common between different packages managers are located in
//! the default package data section.

use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

use crate::package::Validate;

/// Basic structure to hold information regarding a
/// package that are only specific to creating Chocolatey
/// packages.
///
/// ### Examples
///
/// Creating a new data structure and initialize it with different values.
/// ```
/// use pkg_upd::package::chocolatey::ChocolateyMetadata;
///
/// let mut data = ChocolateyMetadata::new().with_authors(&["My-Username"]);
/// data.description = "Some description".into();
///
/// println!("{:#?}", data);
/// ```
///
/// Creating a new data structure and validating the data.
/// ```
/// use pkg_upd::package::chocolatey::ChocolateyMetadata;
/// use pkg_upd::package::Validate;
///
/// let data = ChocolateyMetadata::new();
/// let messages = data.validate_data();
///
/// assert_eq!(messages.len(), 2);
/// ```
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ChocolateyMetadata {
    /// Wether to use force the Chocolatey package
    /// created to use an identifier in a lowercase.
    /// This is something required to be used on
    /// the Chocolatey Community repository.
    #[serde(default = "default_lowercase_id")]
    lowercase_id: bool,

    /// The authors/developers of the software that
    /// the package will be created for.
    authors: Vec<String>,

    /// The description of the software.
    pub description: String,
}

/// Helper function to allow serialization
/// with serde to occur with an lowercase
/// identifier set to `true`.
fn default_lowercase_id() -> bool {
    true
}

impl ChocolateyMetadata {
    /// Helper function to create a new
    /// empty structure of Chocolatey metadata.
    pub fn new() -> ChocolateyMetadata {
        ChocolateyMetadata {
            lowercase_id: default_lowercase_id(),
            authors: vec![],
            description: String::new(),
        }
    }

    /// Returns wether lowercase identifiers
    /// are allowed for this Chocolatey package.
    pub fn lowercase_id(&self) -> bool {
        self.lowercase_id
    }

    /// Returns the authors/developers of the software
    /// the package is created for.
    pub fn authors(&self) -> &[String] {
        self.authors.as_slice()
    }

    /// Allows initializing and setting the Chocolatey metadata
    /// structure with the specified authors/developers of the
    /// software.
    pub fn with_authors<T>(mut self, values: &[T]) -> Self
    where
        T: Display,
    {
        if values.is_empty() {
            panic!("Invalid usage: Authors can not be empty!");
        }

        let mut new_authors = Vec::<String>::with_capacity(values.len());

        for val in values.iter() {
            new_authors.push(val.to_string());
        }

        self.authors = new_authors;

        self
    }
}

impl Validate for ChocolateyMetadata {
    /// Runs basic validation on the metadata for Chocolatey.
    fn validate_data(&self) -> Vec<String> {
        let mut errors = vec![];

        if self.authors.is_empty() || self.authors[0].is_empty() {
            errors.push("There must be at least 1 author specified for the software!".into());
        }
        if self.description.trim().is_empty() {
            errors.push("A description of the software must be provided".into());
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::ChocolateyMetadata;
    use crate::package::Validate;

    #[test]
    fn new_should_create_with_expected_values() {
        let expected = ChocolateyMetadata {
            lowercase_id: true,
            authors: vec![],
            description: String::new(),
        };

        let actual = ChocolateyMetadata::new();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn with_authors_should_set_specified_authors_using_String() {
        let authors = [
            String::from("AdmiringWorm"),
            String::from("Chocolatey-Community"),
        ];

        let actual = ChocolateyMetadata::new().with_authors(&authors);

        assert_eq!(actual.authors, authors);
    }

    #[test]
    fn with_authors_should_set_specified_authors_using_reference_str() {
        let authors = ["AdmiringWorm", "Chocolatey"];

        let actual = ChocolateyMetadata::new().with_authors(&authors);

        assert_eq!(actual.authors, authors);
    }

    #[test]
    #[should_panic]
    fn with_authors_should_panic_on_empty_array() {
        let val: Vec<String> = vec![];
        ChocolateyMetadata::new().with_authors(&val);
    }

    #[test]
    fn authors_should_return_set_values() {
        let expected = ["gep13"];
        let data = ChocolateyMetadata::new().with_authors(&expected);

        let actual = data.authors();

        assert_eq!(actual, expected);
    }

    #[test]
    fn lowercase_id_should_return_set_values() {
        let mut data = ChocolateyMetadata::new();
        assert_eq!(data.lowercase_id, true);
        data.lowercase_id = false;

        let actual = data.lowercase_id();

        assert_eq!(actual, false);
    }

    #[rstest(authors,
		case(vec![]),
		case(vec!["".into()]),
	)]
    fn validate_data_should_add_issue_with_empty_authors(authors: Vec<String>) {
        let mut data = ChocolateyMetadata::new();
        data.authors = authors;
        data.description = "Some description".into();
        let expected = vec!["There must be at least 1 author specified for the software!"];

        let actual = data.validate_data();

        assert_eq!(actual, expected);
    }

    #[rstest(description, case(""), case("   "))]
    fn validate_data_should_add_issue_with_empty_description(description: &str) {
        let mut data = ChocolateyMetadata::new().with_authors(&["AdmiringWorm"]);
        data.description = description.into();
        let expected = vec!["A description of the software must be provided"];

        let actual = data.validate_data();

        assert_eq!(actual, expected)
    }

    #[test]
    fn validate_data_should_not_add_issues_on_valid_data() {
        let mut data = ChocolateyMetadata::new().with_authors(&["eee"]);
        data.description = "Some description".into();
        let actual = data.validate_data();

        assert_eq!(actual, Vec::<String>::new());
    }

    #[test]
    fn validate_data_should_add_multiple_issues_to_vector() {
        let data = ChocolateyMetadata::new();

        let actual = data.validate_data();

        assert_eq!(actual.len(), 2);
    }
}
