use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Severity::Info => "Info",
            Severity::Low => "Low",
            Severity::Medium => "Medium",
            Severity::High => "High",
            Severity::Critical => "Critical",
        };
        f.write_str(s)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    
    // Zero-Length Input
    Empty,
}

impl ParseError {

    pub fn severity(&self) -> Severity {
        todo!("Complete")
    }

    pub fn strings(&self) -> &'static str {
        use ParseError::*;
        match self {
            Empty => "Empty",
        }
    }

    pub fn offset(&self) -> Option<u64> {
        todo!("Complete")
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        todo!("Complete")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Anomaly {
    // Appended Payloads, Installer Archives, and Authenticode
    // can all live here
    Overlay { offset: u64, size: u64, entropy: f64 },

    // Space between sections large enough to hide potential payloads
    Space { offset: u64, size: u64 }, // Not Finished

    // Raw Size far exceeding the virtal size (or vice versa), which
    // is how unpacking stubs reserve room for the decompression image
    SectionSizeMismatch { name: String, raw_size: u64, virtal_size: u64 },

    // Section is writeable and executable, which is almost never emitted
    // by a legitimate toolchain; near-universal in packed samples
    WriteableExecutableSection { name: String },

    // Section is executable but not named like code, or vice versa
    UnexpectedSectionPermissions { name: String, permissions: u32 },

    // Section name is not a known toolchain name (.text/.data/,rdata, etc...)
    // Carries the packer name when recognized (i.e., UPX0, .aspack, etc...)
    UnusualSectionName { name: String, recognized_packer: Option<String> },

    // Duplicate Section Names
    DuplicateSectionName { name: String, count: usize },

    // Section entropy above the compressed/encrypted threshold
    HighEntropySection { name: String, entropy: u64 },

    // Whole entire file suggests packing
    HighEntropyFile { entropy: f64 },

    // Entropy near zero across a large span may suggest padding
    // or a wiped region
    ZeroFilledRegion { offset: u64, size: u64 },

    // Entruy Point is not the first executable region
    EntryPointOutsideCode { rva: u64, section: Option<String> },

    // Entry Point lands in a writeable section
    EntryPointInWriteableSection { rva: u64, section: Option<String> },

    // Entry Point sits in the header or an overlay
    EntryPointOutsideSections { rva: u64, header: Option<String> },

    // Very few imports for the file size
    // Suggests imports that are resolved dynamically at runtime
    SparseImportTable { count: usize },

    // No imports at all
    NoImports,

    // Imports only the runtime-resolution primitives
    DynamicResolutionOnly { functions: Vec<String> },

    // Imports associated with injection, hooking, or anti-analysis
    Imports {
        dll: String,
        function: String,
    },

    IrregularImportLayout {
        rva: u64,
        detail: &'static str
    },

    // Missing or present but with a bad checksum
    // Zeroing the rich header is deliberate of an anti-attribution step
    RichHeaderCheck { detail: &'static str },

    // Rich header contents contradict the linker version
    RichHeaderMismatch {
        rich_linker: String,
        declared_linker: String,
    },

    // Dos Stub differs from standard MSVC Stub
    NonStandardDosStub {
        offset: u64,
        size: u64,
    },

    DebugPathPresent { path: String },

    ImplausibleTimestamp { value: u32 },

    ReproducibleBuildTimestamp { value: u32 },

    VersionInfo { detail: String },

    UnexpectedResourceLanguge { lang_id: u16 },

    // Continue...

}