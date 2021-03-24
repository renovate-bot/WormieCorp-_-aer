// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project
#![windows_subsystem = "console"]

extern crate pkg_upd;

use std::path::PathBuf;
use std::str::FromStr;

use log::info;
use pkg_upd::logging;
use pkg_upd::runners::run_script;

fn main() {
    {
        let log_path = concat!(env!("CARGO_PKG_NAME"), ".log");
        let filter = if cfg!(debug_assertions) {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Info
        };

        logging::setup_logging(&filter, log_path).unwrap();
    }

    let data = pkg_upd::parsers::read_file(
        &PathBuf::from_str("./pkg-upd/test-data/deserialize-full.pkg.toml").unwrap(),
    );
    let mut data = data.unwrap();

    let file = std::env::args().nth(1);

    if let Some(ref file) = file {
        let cwd = std::env::current_dir().unwrap();

        let data = run_script(&cwd, PathBuf::from_str(file).unwrap(), &mut data);
        info!("{:?}", data);
    }

    info!("Hello, world!");
}
