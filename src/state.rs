use std::ffi::OsString;
use std::path::{Path, PathBuf};
use tiny_keccak::keccak512;

pub struct ActiveMetadata {
    path: PathBuf,
    sum: [u8; 64],
}

pub struct ProjectMetadata {
    path: PathBuf,
}

impl ActiveMetadata {
    pub fn new(path: PathBuf, data: &[u8]) -> ActiveMetadata {
        ActiveMetadata {
            path,
            sum: keccak512(data),
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_dir(&self) -> Option<PathBuf> {
        self.path.parent().map(|p| p.to_path_buf())
    }

    pub fn is_same_as(&self, data: &[u8]) -> bool {
        &keccak512(data)[..] == &self.sum[..]
    }

    pub fn set_sum(&mut self, data: &[u8]) {
        self.sum = keccak512(data);
    }
}

impl ProjectMetadata {
    pub fn new(path: PathBuf) -> ProjectMetadata {
        ProjectMetadata { path }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_name(&self) -> Option<OsString> {
        self.path.file_name().map(|p| p.to_os_string())
    }
}
