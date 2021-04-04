// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

pub mod logging;

use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

use md5::Md5;
use sha1::Sha1;
use sha2::digest::generic_array::ArrayLength;
use sha2::{Digest, Sha256, Sha512};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum ChecksumType {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

impl FromStr for ChecksumType {
    type Err = &'static str;

    fn from_str(val: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let val: &str = &val.trim().to_lowercase();

        match val {
            "md5" => Ok(ChecksumType::Md5),
            "sha1" => Ok(ChecksumType::Sha1),
            "sha256" => Ok(ChecksumType::Sha256),
            "sha512" => Ok(ChecksumType::Sha512),
            _ => Err("The value is not a supported checksum type"),
        }
    }
}

impl Display for ChecksumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ChecksumType::Md5 => f.write_str("MD5"),
            ChecksumType::Sha1 => f.write_str("SHA1"),
            ChecksumType::Sha256 => f.write_str("SHA256"),
            ChecksumType::Sha512 => f.write_str("SHA512"),
        }
    }
}

impl Default for ChecksumType {
    fn default() -> Self {
        Self::Sha256
    }
}

impl ChecksumType {
    pub fn variants() -> &'static [ChecksumType] {
        static VARIANTS: &[ChecksumType] = &[
            ChecksumType::Md5,
            ChecksumType::Sha1,
            ChecksumType::Sha256,
            ChecksumType::Sha512,
        ];

        VARIANTS
    }

    pub fn variants_str() -> &'static [&'static str] {
        static VARIANTS: &[&str] = &["MD5", "SHA1", "SHA256", "SHA512"];

        VARIANTS
    }

    pub fn generate(&self, path: &Path) -> Result<String, std::io::Error> {
        generate_checksum(path, self)
    }
}

fn generate_checksum(path: &Path, checksum_type: &ChecksumType) -> Result<String, std::io::Error> {
    match checksum_type {
        ChecksumType::Md5 => generate_checksum_from_hasher(Md5::new(), path),
        ChecksumType::Sha1 => generate_checksum_from_hasher(Sha1::new(), path),
        ChecksumType::Sha256 => generate_checksum_from_hasher(Sha256::new(), path),
        ChecksumType::Sha512 => generate_checksum_from_hasher(Sha512::new(), path),
    }
}

fn generate_checksum_from_hasher<T: Digest + Write>(
    mut hasher: T,
    path: &Path,
) -> Result<String, std::io::Error>
where
    <T as Digest>::OutputSize: Add,
    <<T as Digest>::OutputSize as Add>::Output: ArrayLength<u8>,
{
    let mut f = File::open(path)?;
    std::io::copy(&mut f, &mut hasher)?;
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}
