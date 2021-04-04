// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

pub mod chocolatey;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub struct PackageUpdateData {
    #[cfg(feature = "chocolatey")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chocolatey")))]
    chocolatey: Option<chocolatey::ChocolateyUpdaterData>,
}

impl PackageUpdateData {
    pub fn new() -> PackageUpdateData {
        PackageUpdateData {
            #[cfg(feature = "chocolatey")]
            chocolatey: None,
        }
    }

    #[cfg(feature = "chocolatey")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chocolatey")))]
    pub fn chocolatey(&self) -> &Option<chocolatey::ChocolateyUpdaterData> {
        &self.chocolatey
    }

    #[cfg(feature = "chocolatey")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chocolatey")))]
    pub fn set_chocolatey(&mut self, choco: chocolatey::ChocolateyUpdaterData) {
        self.chocolatey = Some(choco);
    }
}
