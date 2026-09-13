// SPDX-License-Identifier: BSD-3-Clause

use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanupOutcome {
    Cleaned,
    NothingToClean,
}

const MANIFEST_NAME: &str = ".nightkrawler-manifest";
const QUARANTINE_MAGIC: &[u8; 4] = b"NKQ1";

pub fn cleanup_quarantine(quarantine_path: &Path) -> std::io::Result<CleanupOutcome> {
    if !quarantine_path.exists() {
        return Ok(CleanupOutcome::NothingToClean);
    }

    let mut entries = fs::read_dir(quarantine_path)?;

    if entries.next().is_none() {
        fs::remove_dir(quarantine_path)?;

        return Ok(CleanupOutcome::NothingToClean);
    }

    let canonical = fs::canonicalize(quarantine_path)?;

    if canonical == Path::new("/") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "refusing to clean the filesystem root",
        ));
    }

    if let Some(home) = std::env::var_os("HOME") {
        let home = fs::canonicalize(home)?;

        if canonical == home {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "refusing to clean the home directory",
            ));
        }
    }

    if let Ok(current) = std::env::current_dir() {
        if let Ok(current) = fs::canonicalize(current) {
            if canonical == current {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "refusing to clean the current working directory",
                ));
            }
        }
    }

    let manifest_path = canonical.join(MANIFEST_NAME);

    let mut file = fs::File::open(&manifest_path)?;

    let mut magic = [0u8; 4];

    file.read_exact(&mut magic)?;

    if &magic != QUARANTINE_MAGIC {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "directory is not a recognized Nightkrawler quarantine",
        ));
    }

    fs::remove_dir_all(&canonical)?;

    Ok(CleanupOutcome::Cleaned)
}
