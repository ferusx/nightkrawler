// SPDX-License-Identifier: BSD-3-Clause

use crate::model::{CandidateFile, Options};
use std::fs;
use std::path::Path;

pub fn collect_regular_files(path: &Path, options: &Options, files: &mut Vec<CandidateFile>) {
    if should_exclude_path(path, options) {
        return;
    }

    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,

        Err(error) => {
            eprintln!("nightkrawler: unable to read {}: {}", path.display(), error);

            return;
        }
    };

    if metadata.file_type().is_symlink() {
        return;
    }

    if metadata.is_file() {
        if metadata.len() >= options.min_size {
            files.push(CandidateFile {
                path: path.to_path_buf(),
                size: metadata.len(),
            });
        }

        return;
    }

    if !metadata.is_dir() {
        return;
    }

    if is_dotdir(path) && !options.force_dotdirs {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,

        Err(error) => {
            eprintln!("nightkrawler: unable to read {}: {}", path.display(), error);

            return;
        }
    };

    for result in entries {
        let Ok(dir_entry) = result else {
            continue;
        };

        let entry_path = dir_entry.path();

        let Ok(entry_metadata) = fs::symlink_metadata(&entry_path) else {
            continue;
        };

        if entry_metadata.file_type().is_symlink() {
            continue;
        }

        if entry_metadata.is_file() {
            if entry_metadata.len() >= options.min_size {
                files.push(CandidateFile {
                    path: entry_path,
                    size: entry_metadata.len(),
                });
            }

            continue;
        }

        if options.recursive && entry_metadata.is_dir() {
            collect_regular_files(&entry_path, options, files);
        }
    }
}

fn should_exclude_path(path: &Path, options: &Options) -> bool {
    let canonical_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    if let Some(quarantine_path) = options.quarantine_path.as_ref() {
        let canonical_quarantine =
            fs::canonicalize(quarantine_path).unwrap_or_else(|_| quarantine_path.clone());

        if canonical_path.starts_with(canonical_quarantine) {
            return true;
        }
    }
    options.excludes.iter().any(|exclude| {
        let canonical_exclude = fs::canonicalize(exclude).unwrap_or_else(|_| exclude.clone());

        canonical_path.starts_with(canonical_exclude)
    })
}

fn is_dotdir(path: &Path) -> bool {
    let Some(file_name) = path.file_name() else {
        return false;
    };

    let name = file_name.to_string_lossy();

    name.starts_with('.')
}
