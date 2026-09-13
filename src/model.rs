// SPDX-License-Identifier: BSD-3-Clause

use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum KeepPolicy {
    Default,
    HighestVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathClass {
    Ordinary,
    Generated,
    Protected,
}

#[derive(Debug)]
pub struct Options {
    pub root_path: PathBuf,
    pub recursive: bool,
    pub min_size: u64,
    pub force_dotdirs: bool,
    pub allow_protected: bool,
    pub delete: bool,
    pub trash: bool,
    pub excludes: Vec<PathBuf>,
    pub protected_paths: Vec<PathBuf>,
    pub prefer_paths: Vec<PathBuf>,
    pub keep_policy: KeepPolicy,
    pub quarantine_path: Option<PathBuf>,
    pub report_path: Option<PathBuf>,
    pub interactive: bool,
    pub use_colors: bool,
}

#[derive(Debug, Clone)]
pub struct CandidateFile {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug)]
pub struct DuplicateGroup {
    pub keep: CandidateFile,
    pub remove: Vec<CandidateFile>,
}

#[derive(Debug, Default)]
pub struct QuarantineSummary {
    pub copied: usize,
    pub failed: usize,
    pub skipped: usize,
    pub records: Vec<QuarantineRecord>,
}

#[derive(Debug, Default)]
pub struct TrashSummary {
    pub kept: usize,
    pub trashed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub trashed_space: u64,
}

#[derive(Debug, Default)]
pub struct DeleteSummary {
    pub kept: usize,
    pub deleted: usize,
    pub failed: usize,
    pub skipped: usize,
    pub deleted_space: u64,
}

#[derive(Debug, Clone)]
pub struct QuarantineRecord {
    pub original_path: PathBuf,
    pub quarantine_path: PathBuf,
    pub size: u64,
}

#[derive(Debug, Default)]
pub struct RestoreSummary {
    pub restored: usize,
    pub failed: usize,
    pub skipped: usize,
    pub restored_space: u64,
}

#[derive(Debug)]
pub enum Command {
    Scan(Options),
    Restore(PathBuf),
    Cleanup(PathBuf),
}
