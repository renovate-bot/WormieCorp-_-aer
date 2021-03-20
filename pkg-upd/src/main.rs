// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project
#![windows_subsystem = "console"]

extern crate pkg_upd;

use std::path::PathBuf;
use std::str::FromStr;

use pkg_upd::runners::run_script;

fn main() {
    flexi_logger::Logger::with_str("trace").start().unwrap();

    let data = pkg_upd::parsers::read_file(
        &PathBuf::from_str("./pkg-upd/test-data/deserialize-full.pkg.toml").unwrap(),
    );
    let mut data = data.unwrap();

    let file = std::env::args().nth(1);

    if let Some(ref file) = file {
        let cwd = std::env::current_dir().unwrap();

        let data = run_script(&cwd, PathBuf::from_str(file).unwrap(), &mut data);
        println!("{:?}", data);
    }

    println!("Hello, world!");
}
