// SPDX-License-Identifier: BSD-3-Clause

use crate::model::RestoreSummary;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Read};
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

const MANIFEST_NAME: &str = ".nightkrawler-manifest";
const QUARANTINE_MAGIC: &[u8; 4] = b"NKQ1";

pub fn restore_quarantine(
    quarantine_path: &Path,
) -> std::io::Result<RestoreSummary> {
    let records = read_quarantine_manifest(quarantine_path)?;

    let mut summary = RestoreSummary::default();

    for record in records {
        if record.original_path.exists() {
            summary.skipped += 1;
            continue;
        }

        if !record.quarantine_path.exists() {
            summary.failed += 1;
            continue;
        }

        if let Some(parent) = record.original_path.parent() {
            if fs::create_dir_all(parent).is_err() {
                summary.failed += 1;
                continue;
            }
        }

        match fs::copy(
            &record.quarantine_path,
            &record.original_path,
        ) {
            Ok(_) => {
                summary.restored += 1;

                summary.restored_space =
                    summary.restored_space.saturating_add(record.size);
            }

            Err(_) => {
                summary.failed += 1;
            }
        }
    }

    Ok(summary)
}

fn read_quarantine_manifest(
    quarantine_path: &Path,
) -> io::Result<Vec<crate::model::QuarantineRecord>> {
    let manifest_path = quarantine_path.join(MANIFEST_NAME);

    let mut file = fs::File::open(manifest_path)?;

    let mut magic = [0u8; 4];

    file.read_exact(&mut magic)?;

    if &magic != QUARANTINE_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid Nightkrawler quarantine manifest",
        ));
    }

    let mut records = Vec::new();

    loop {
        let Some(original_path) = read_path_or_eof(&mut file)? else {
            break;
        };

        let quarantine_path = read_path(&mut file)?;

        let mut size_bytes = [0u8; 8];

        file.read_exact(&mut size_bytes)?;

        let size = u64::from_le_bytes(size_bytes);

        records.push(crate::model::QuarantineRecord {
            original_path,
            quarantine_path,
            size,
        });
    }

    Ok(records)
}

fn read_path_or_eof(file: &mut fs::File) -> io::Result<Option<PathBuf>> {
    let mut length_bytes = [0u8; 8];

    match file.read_exact(&mut length_bytes) {
        Ok(()) => {}

        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => {
            return Ok(None);
        }

        Err(error) => {
            return Err(error);
        }
    }

    read_path_with_length(file, length_bytes).map(Some)
}

fn read_path(file: &mut fs::File) -> io::Result<PathBuf> {
    let mut length_bytes = [0u8; 8];

    file.read_exact(&mut length_bytes)?;

    read_path_with_length(file, length_bytes)
}

fn read_path_with_length(file: &mut fs::File, length_bytes: [u8; 8]) -> io::Result<PathBuf> {
    let length = u64::from_le_bytes(length_bytes);

    let length = usize::try_from(length).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid path length in quarantine manifest",
        )
    })?;

    let mut bytes = vec![0u8; length];

    file.read_exact(&mut bytes)?;

    Ok(PathBuf::from(OsString::from_vec(bytes)))
}
