use crate::binary::*;
use crate::windows::binary::*;
use crate::linux::binary::*;
use crate::mac::binary::*;

use std::fmt::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Complete,
    Incomplete,
}

impl fmt::Display for Status {

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Full,
    Summary,
}

impl fmt::Display for Mode {

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Review {
    Normal,
    Review,
}

impl fmt::Display for Review {

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileHashes {
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub sha512: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileMetadata {
    pub name: String,
    pub path: Option<String>,
    pub size: u64,
    pub hashes: FileHashes,
}

