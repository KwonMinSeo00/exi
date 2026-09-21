/*
    Core PE32/PE32+ headers and their shared format expectations.
    Windows version profiles select these rules where applicable.

    Reference: https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
*/

// DOS header and PE signature.
pub mod dos_header;
pub mod pe_signature;

// COFF and optional headers.
pub mod coff_header;
pub mod optional_header;
pub mod optional_header32;
pub mod optional_header64;

// Header tables.
pub mod data_directories;
pub mod section_table;
