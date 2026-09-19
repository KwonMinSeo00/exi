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

