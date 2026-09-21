use std::fmt;

macro_rules! address_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub u64);

        impl $name {
            pub const ZERO: Self = Self(0);

            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            pub const fn get(self) -> u64 {
                self.0
            }

            pub fn checked_add(self, offset: u64) -> Option<Self> {
                self.0.checked_add(offset).map(Self)
            }

            pub fn checked_sub(self, offset: u64) -> Option<Self> {
                self.0.checked_sub(offset).map(Self)
            }
        }

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                Self(value as u64)
            }
        }

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                Self(value as u64)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({:#x})", stringify!($name), self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }
    };
}

address_type!(FileOffset);
address_type!(Rva);
address_type!(VirtualAddress);

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileRange {
    pub offset: FileOffset,
    pub size: u64,
}

impl fmt::Display for FileRange {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!("Write out FileRange")
    }
}

impl FileRange {
    
}