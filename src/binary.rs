use std::fmt;
use std::sync::Arc;

// This file defines common data types
// and operations on their stored values

// Raw fields may describe an invalid range. Therefore, perform
// bounds check against the input to verify the range before reading
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileRange {
    
}

impl fmt::Display for FileRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!(" Implement File Range ")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RvaRange {

}

impl fmt::Display for RvaRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!(" Implement RVA Range ")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualRange {

}

impl fmt::Display for VirtualRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!(" Implement Virtual Range ")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BaseImageAddress {
    pub address: u64, // Get Base Image Address
}

impl fmt::Display for BaseImageAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.address)
        // 0x supplies the prefix
        // {:X} formated the stored number
    }
}
