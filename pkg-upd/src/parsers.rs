// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

use std::fs::File;
use std::io::{BufReader, Error as IoError, ErrorKind, Read};
use std::path::Path;

use log::warn;
use pkg_data::prelude::*;

pub mod errors;
pub mod toml;

/// Parsers implementing this trait are able to read and transform a specific
/// structure to the [PackageData] type.
pub trait DataReader {
    /// Function to decide if the implemented structure can handle a certain
    /// file (usually by file extension).
    fn can_handle_file(&self, path: &Path) -> bool;

    /// Read and Deserialize the specified file, calling the implemented
    /// structure that handle the Deserialization.
    fn read_file(&self, path: &Path) -> Result<PackageData, errors::ParserError> {
        if !self.can_handle_file(path) {
            let error = IoError::new(
                ErrorKind::InvalidData,
                format!("The file '{}' is not a supported type.", path.display()),
            );
            warn!("{}", error);
            return Err(errors::ParserError::Loading(error));
        }

        if !path.exists() {
            let error = IoError::new(
                ErrorKind::NotFound,
                format!("The file '{}' could not be found!", path.display()),
            );
            warn!("{}", error);
            return Err(errors::ParserError::Loading(error));
        }

        let file = match File::open(path) {
            Ok(f) => f,
            Err(error) => return Err(errors::ParserError::Loading(error)),
        };
        let mut buffer = BufReader::new(file);

        self.read_data(&mut buffer)
    }

    /// Read the specifed buffer and return either the parsed package data, or
    /// an error if one occurs.
    fn read_data<T: Read>(&self, reader: &mut T) -> Result<PackageData, errors::ParserError>;
}

pub fn read_file(path: &Path) -> Result<PackageData, errors::ParserError> {
    let parsers = [toml::TomlParser];

    for parser in &parsers {
        let data = parser.read_file(path);
        if let Ok(data) = data {
            return Ok(data);
        } else if parser.can_handle_file(path) {
            return data;
        }
    }

    Err(errors::ParserError::NoParsers(path.to_owned()))
}
