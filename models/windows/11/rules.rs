// Windows 11 scaffold. This shared PE check is not a measured loader model.

use super::super::Rule;
use super::super::headers::dos_header::DOS_SIGNATURE;

pub const RULES: [Rule; 1] = [DOS_SIGNATURE];
