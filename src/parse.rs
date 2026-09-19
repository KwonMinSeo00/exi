use std::fmt;

pub const MaxFileSize: u64 = 256 * 1024 * 1024;

pub fn GetFileRange() {
    // Determine the total byte length of the input file.
    // Represent the entire file as a FileRange starting at offset zero.
    // Return the range to the caller.
}

pub fn GetRVARange() {
    // Determine the region's starting RVA and size from the file data.
    // Store the start and size in an RvaRange.
    // Return the range to the caller.
}
pub fn GetBaseImageAddress() { 
    // Read the base image address from the executable's format-specific data.
    // Store the extracted value in a BaseImageAddress.
    // Return the address to the caller.
}
