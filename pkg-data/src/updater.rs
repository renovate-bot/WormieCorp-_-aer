// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

pub mod chocolatey;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
#[non_exhaustive]
pub struct PackageUpdateData {
    chocolatey: Option<chocolatey::ChocolateyUpdaterData>,
}

impl PackageUpdateData {
    pub fn new() -> PackageUpdateData {
        PackageUpdateData { chocolatey: None }
    }

    pub fn chocolatey(&self) -> &Option<chocolatey::ChocolateyUpdaterData> {
        &self.chocolatey
    }

    pub fn set_chocolatey(&mut self, choco: chocolatey::ChocolateyUpdaterData) {
        self.chocolatey = Some(choco);
    }
}
