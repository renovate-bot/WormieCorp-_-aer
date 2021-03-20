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

macro_rules! call_runners {
    ($work_dir:ident,$script_path:ident,$data:ident,$($runner:expr=>$feature:literal),+) => {
        let script_path = $script_path.canonicalize().unwrap();
        let work_dir = $work_dir.canonicalize().unwrap();
        $(
            #[cfg(feature = $feature)]
            if $runner.can_run(&script_path) {
                return $runner.run(&work_dir, script_path, $data);
            }
        )*
    };
}

pub fn run_script<T: RunnerCombiner + Debug>(
    work_dir: &Path,
    script_path: PathBuf,
    data: &mut T,
) -> Result<(), String> {
    if !work_dir.exists() {
        if let Err(err) = std::fs::create_dir_all(work_dir) {
            let msg = format!("Failed to create work directory: '{}'", err);
            error!("{}", msg);
            return Err(msg);
        }
    }

    let work_dir = &if work_dir.is_absolute() {
        work_dir.to_path_buf()
    } else {
        work_dir.canonicalize().unwrap()
    };

    if !work_dir.is_dir() {
        return Err(format!(
            "The specified directory '{}' is not a directory!",
            work_dir.display()
        ));
    }

    call_runners!(
        work_dir,
        script_path,
        data,
        powershell::PowershellRunner => "powershell"
    );

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

#[cfg(test)]
mod tests {
    use std::fs::{create_dir_all, File};
    use std::io::{BufWriter, Write};

    use super::*;

    fn write_file(content: &[u8], file_path: &Path) {
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent).unwrap();
        }
        let file = File::create(PathBuf::from(file_path)).unwrap();
        let mut writer = BufWriter::new(file);

        writer.write_all(content).unwrap();
    }

    #[test]
    fn get_license_should_get_license_expression() {
        let mut data = RunnerData::new();
        data.insert("expr", "GPL-3.0");

        let result = get_license(data);

        assert_eq!(result, LicenseType::Expression("GPL-3.0".into()));
    }

    #[test]
    fn get_license_should_get_license_url() {
        const EXPECTED: &str = "https://opensource.org/licenses/MIT";
        let mut data = RunnerData::new();
        data.insert("url", EXPECTED);

        let result = get_license(data);

        assert_eq!(result, LicenseType::Location(Url::parse(EXPECTED).unwrap()));
    }

    #[test]
    fn get_license_should_get_license_expression_and_url() {
        const EXPECTED_EXPR: &str = "Apache-2.0";
        const EXPECTED_URL: &str = "https://opensource.org/licenses/Apache-2.0";
        let mut data = RunnerData::new();
        data.insert("url", EXPECTED_URL);
        data.insert("expr", EXPECTED_EXPR);

        let result = get_license(data);

        assert_eq!(
            result,
            LicenseType::ExpressionAndLocation {
                expression: EXPECTED_EXPR.into(),
                url: Url::parse(EXPECTED_URL).unwrap()
            }
        );
    }

    #[test]
    fn get_license_should_return_no_license_on_invalid_data() {
        let mut data = RunnerData::new();
        data.insert("project", "some project");
        let result = get_license(data);

        assert_eq!(result, LicenseType::None);
    }

    #[test]
    fn run_script_should_run_powershell_scripts() {
        const SCRIPT: &[u8] = b"param($data)
        Write-Host \"Hello world\"";
        let file_path = PathBuf::from("./test-files/test-path.ps1");
        write_file(SCRIPT, &file_path);

        let result = run_script(
            &PathBuf::from("."),
            PathBuf::from(file_path),
            &mut PackageData::new("test-package"),
        );

        assert_eq!(result, Ok(()))
    }

    #[test]
    fn run_script_should_return_error_on_unknown_script() {
        let file_path = PathBuf::from("./test-files/test-path.qs");
        write_file(b"Test", &file_path);

        let result = run_script(
            &PathBuf::from(".").canonicalize().unwrap(),
            file_path.clone(),
            &mut PackageData::new("test-package"),
        );

        assert_eq!(
            result,
            Err(format!(
                "No supported runner was found for '{}'",
                file_path.display()
            ))
        );
    }

    #[test]
    fn run_script_should_return_error_when_work_dir_is_a_file() {
        let work_dir = PathBuf::from("Cargo.toml");
        let result = run_script(
            &work_dir,
            PathBuf::from("test"),
            &mut PackageData::new("test-data"),
        );

        assert_eq!(
            result,
            Err(format!(
                "The specified directory '{}' is not a directory!",
                work_dir.canonicalize().unwrap().display()
            ))
        );
    }

    #[test]
    fn run_script_should_create_work_directory_if_not_exists() {
        let path = PathBuf::from("test-files/work_dir");

        if path.exists() {
            let _ = std::fs::remove_dir(&path);
        }

        let _ = run_script(
            &path,
            PathBuf::from("Cargo.toml"),
            &mut PackageData::new("test-package"),
        );

        assert!(path.exists());
    }
}
