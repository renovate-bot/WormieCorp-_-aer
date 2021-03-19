// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ChocolateyUpdaterType {
    None,
    Installer,
    Archive,
}

impl Default for ChocolateyUpdaterType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ChocolateyParseUrl {
    UrlWithRegex { url: Url, regex: String },
    Url(Url),
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
#[non_exhaustive]
pub struct ChocolateyUpdaterData {
    #[serde(default)]
    pub embedded: bool,
    #[serde(default, rename = "type")]
    pub _type: ChocolateyUpdaterType,
    pub parse_url: Option<ChocolateyParseUrl>,

    regexes: HashMap<String, String>,
}

impl ChocolateyUpdaterData {
    pub fn new() -> ChocolateyUpdaterData {
        ChocolateyUpdaterData {
            embedded: false,
            _type: ChocolateyUpdaterType::default(),
            parse_url: None,
            regexes: HashMap::new(),
        }
    }

    pub fn regexes(&self) -> &HashMap<String, String> {
        &self.regexes
    }

    pub fn add_regex(&mut self, name: &str, value: &str) {
        self.regexes.insert(name.into(), value.into());
    }

    pub fn set_regexes(&mut self, values: HashMap<String, String>) {
        self.regexes = values;
    }
}
