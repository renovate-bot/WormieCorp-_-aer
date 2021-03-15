// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    Loading(std::io::Error),
    Deserialize(String),
    Other { inner: Box<dyn Error> },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::Loading(err) => err.fmt(f),
            ParserError::Deserialize(s) => s.fmt(f),
            ParserError::Other { inner } => inner.fmt(f),
        }
    }
}

impl Error for ParserError {}
