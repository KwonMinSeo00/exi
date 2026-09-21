/*
    Shared PE32/PE32+ structures and relationships outside the core headers.

    This is a scaffold for future rules, not a claim of implemented coverage.
    Common format expectations belong here; measured loader differences belong
    in the Windows version profiles. Unusual content alone is not malicious.

    References:
    - PE layout: <https://learn.microsoft.com/en-us/windows/win32/debug/pe-format>
    - Load config: <https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_load_config_directory64>
    - Unwind data: <https://learn.microsoft.com/en-us/cpp/build/exception-handling-x64>
    - Managed metadata: <https://learn.microsoft.com/en-us/dotnet/standard/metadata-and-self-describing-components>
*/

// DOS stub and optional toolchain metadata.
pub mod dos_stub;
pub mod rich_header;

// Section contents; section headers are declared in models::headers.
pub mod section_data;

// Conventional data-directory slots, in index order (0 through 15).
pub mod exports;
pub mod imports;
pub mod resources;
pub mod exceptions;
pub mod certificates;
pub mod base_relocations;
pub mod debug;
pub mod architecture_directory;
pub mod global_pointer;
pub mod tls;
pub mod load_config;
pub mod bound_imports;
pub mod import_address_table;
pub mod delay_imports;
pub mod clr_header;
pub mod reserved_directory;

// Structures referenced by the directories.
pub mod unwind_info;
pub mod resource_data;
pub mod version_info;
pub mod manifest;
pub mod debug_data;
pub mod clr_metadata;
pub mod managed_resources;
pub mod managed_code;

// Legacy COFF structures; not mandatory PE image components.
pub mod coff_relocations;
pub mod coff_symbols;
pub mod coff_strings;
pub mod coff_line_numbers;

// Relationships across structures and additional file data.
pub mod address_mapping;
pub mod alignment;
pub mod file_layout;
pub mod overlay;
pub mod checksum;
pub mod authenticode;
