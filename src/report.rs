use crate::binary::*;
use crate::windows::binary::*;
use crate::linux::binary::*;
use crate::mac::binary::*;

use std::fmt::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Full,
    Summary,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Full => "Full Report",
            Self::Summary => "Summarized Report",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Review {
    Normal,
    Review,
}

impl fmt::Display for Review {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Normal => " No Further Review ",
            Self::Review => " Further Review Needed ",
        })
    }
}

