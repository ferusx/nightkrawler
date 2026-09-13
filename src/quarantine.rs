// SPDX-License-Identifier: BSD-3-Clause

use crate::model::{CandidateFile, DuplicateGroup, QuarantineSummary};
use std::fs;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

const MANIFEST_NAME: &str = ".nightkrawler-manifest";
const QUARANTINE_MAGIC: &[u8; 4] = b"NKQ1";

pub fn quarantine_was_complete(
    duplicate_groups: &[DuplicateGroup],
    summary: &QuarantineSummary,
) -> bool {
    summary.failed == 0
        && summary.skipped == 0
        && summary.copied == total_removable_count(duplicate_groups)
}

fn total_removable_count(duplicate_groups: &[DuplicateGroup]) -> usize {
    duplicate_groups
        .iter()
        .map(|group| group.remove.len())
        .sum()
}

pub fn total_removable_size(duplicate_groups: &[DuplicateGroup]) -> u64 {
    duplicate_groups
        .iter()
        .flat_map(|group| group.remove.iter())
        .map(|file| file.size)
        .sum()
}

pub fn copy_file_to_quarantine(
    file: &CandidateFile,
    root_path: &Path,
    quarantine_path: &Path,
) -> std::io::Result<PathBuf> {
    let relative_path = file.path.strip_prefix(root_path).unwrap_or(&file.path);

    let destination = unique_quarantine_destination(quarantine_path.join(relative_path));

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(&file.path, &destination)?;

    Ok(destination)
}

pub fn unique_quarantine_destination(destination: PathBuf) -> PathBuf {
    if !destination.exists() {
        return destination;
    }

    let parent = destination
        .parent()
        .map(|path| path.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let file_name = destination
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "file".to_string());

    for counter in 1.. {
        let candidate = parent.join(format!("{}.nightkrawler-{}", file_name, counter));

        if !candidate.exists() {
            return candidate;
        }
    }

    unreachable!()
}

pub fn write_quarantine_manifest(
    quarantine_path: &Path,
    records: &[crate::model::QuarantineRecord],
) -> io::Result<()> {
    let manifest_path = quarantine_path.join(MANIFEST_NAME);

    let mut file = fs::File::create(manifest_path)?;

    file.write_all(QUARANTINE_MAGIC)?;

    for record in records {
        write_path(&mut file, &record.original_path)?;

        write_path(&mut file, &record.quarantine_path)?;

        file.write_all(&record.size.to_le_bytes())?;
    }

    file.flush()
}

fn write_path(file: &mut fs::File, path: &Path) -> io::Result<()> {
    let bytes = path.as_os_str().as_bytes();

    let length = u64::try_from(bytes.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "path is too long for quarantine manifest",
        )
    })?;

    file.write_all(&length.to_le_bytes())?;

    file.write_all(bytes)
}
