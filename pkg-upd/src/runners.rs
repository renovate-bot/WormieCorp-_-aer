// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

use log::error;
use pkg_data::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(feature = "powershell")]
pub mod powershell;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RunnerData {
    #[cfg_attr(feature = "serde", serde(default, flatten))]
    data: HashMap<String, RunnerChildType>,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(untagged))]
pub enum RunnerChildType {
    Data(String),
    Child(RunnerData),
}

impl RunnerData {
    fn new() -> RunnerData {
        RunnerData {
            data: HashMap::new(),
        }
    }

    fn insert<T: ToString>(&mut self, key: &'static str, value: T) {
        let val = value.to_string();

        self.data.insert(key.into(), RunnerChildType::Data(val));
    }

    fn insert_child(&mut self, key: &'static str, value: RunnerData) {
        self.data.insert(key.into(), RunnerChildType::Child(value));
    }
}

pub trait ScriptRunner {
    fn can_run(&self, script_path: &Path) -> bool;
    fn run<'a, T: RunnerCombiner + Debug>(
        &self,
        work_dir: &'a Path,
        script_path: PathBuf,
        data: &'a mut T,
    ) -> Result<(), String>;
}

pub fn run_script<T: RunnerCombiner + Debug>(
    work_dir: &Path,
    script_path: PathBuf,
    data: &mut T,
) -> Result<(), String> {
    let work_dir = &work_dir.canonicalize().unwrap();
    if !work_dir.exists() {
        if let Err(err) = std::fs::create_dir_all(work_dir) {
            let msg = format!("Failed to create work directory: '{}'", err);
            error!("{}", msg);
            return Err(msg);
        }
    }
    if !work_dir.is_dir() {
        return Err(format!(
            "The specified directory '{}' is not a directory!",
            work_dir.display()
        ));
    }

    let runners = [
        #[cfg(feature = "powershell")]
        powershell::PowershellRunner,
    ];

    let script_path = script_path.canonicalize().unwrap();

    for runner in runners.iter() {
        if runner.can_run(&script_path) {
            return runner.run(&work_dir, script_path, data);
        }
    }

    Err(format!(
        "No supported runner was found for '{}'",
        script_path.display()
    ))
}

pub trait RunnerCombiner {
    fn to_runner_data(&self) -> RunnerData;

    fn from_runner_data(&mut self, data: RunnerData);
}

impl RunnerCombiner for pkg_data::PackageData {
    fn to_runner_data(&self) -> RunnerData {
        let mut data = RunnerData::new();

        {
            let metadata = self.metadata();
            data.insert("id", metadata.id());
            data.insert("url", metadata.project_url());

            let license = metadata.license();
            let mut license_child = RunnerData::new();

            if let Some(url) = license.license_url() {
                license_child.insert("url", url);
            }

            match license {
                LicenseType::Expression(expression)
                | LicenseType::ExpressionAndLocation { expression, .. } => {
                    license_child.insert("expr", expression);
                }
                _ => {}
            }

            data.insert_child("license", license_child);
        }

        data
    }

    fn from_runner_data(&mut self, data: RunnerData) {
        for (key, val) in data.data {
            match val {
                RunnerChildType::Data(val) => match key.trim() {
                    "project_url" => self.metadata_mut().set_project_url(&val),
                    "summary" => self.metadata_mut().summary = val,
                    _ => {}
                },
                RunnerChildType::Child(val) => {
                    if let "license" = key.trim() {
                        self.metadata_mut().set_license(get_license(val));
                    }
                }
            }
        }
    }
}

fn get_license(values: RunnerData) -> LicenseType {
    let mut license = LicenseType::None;

    for (key, val) in values.data {
        if let RunnerChildType::Data(val) = val {
            match key.trim() {
                "url" => {
                    license = match license {
                        LicenseType::Expression(expression)
                        | LicenseType::ExpressionAndLocation { expression, .. } => {
                            LicenseType::ExpressionAndLocation {
                                url: Url::parse(&val).unwrap(),
                                expression,
                            }
                        }
                        _ => LicenseType::Location(Url::parse(&val).unwrap()),
                    }
                }
                "expr" => {
                    license = match license {
                        LicenseType::Location(url)
                        | LicenseType::ExpressionAndLocation { url, .. } => {
                            LicenseType::ExpressionAndLocation {
                                url,
                                expression: val,
                            }
                        }
                        _ => LicenseType::Expression(val),
                    }
                }
                _ => {}
            }
        }
    }

    license
}
