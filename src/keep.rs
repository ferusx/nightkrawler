// SPDX-License-Identifier: BSD-3-Clause

use crate::model::{CandidateFile, DuplicateGroup, KeepPolicy};
use std::fs;
use std::path::{Path, PathBuf};

pub fn duplicate_group_from_files(
    mut files: Vec<CandidateFile>,
    keep_policy: KeepPolicy,
    prefer_paths: &[PathBuf],
) -> DuplicateGroup {
    files.sort_by(|a, b| compare_keep_candidates(a, b, keep_policy, prefer_paths));

    let keep = files.remove(0);

    DuplicateGroup {
        keep,
        remove: files,
    }
}

fn compare_keep_candidates(
    a: &CandidateFile,
    b: &CandidateFile,
    keep_policy: KeepPolicy,
    prefer_paths: &[PathBuf],
) -> std::cmp::Ordering {
    let a_preferred = preferred_path_rank(&a.path, prefer_paths);

    let b_preferred = preferred_path_rank(&b.path, prefer_paths);

    match (a_preferred, b_preferred) {
        (Some(a_rank), Some(b_rank)) => a_rank
            .cmp(&b_rank)
            .then_with(|| compare_keep_policy_candidates(a, b, keep_policy)),

        (Some(_), None) => std::cmp::Ordering::Less,

        (None, Some(_)) => std::cmp::Ordering::Greater,

        (None, None) => compare_keep_policy_candidates(a, b, keep_policy),
    }
}

fn compare_keep_policy_candidates(
    a: &CandidateFile,
    b: &CandidateFile,
    keep_policy: KeepPolicy,
) -> std::cmp::Ordering {
    match keep_policy {
        KeepPolicy::HighestVersion => {
            let a_version = best_version_in_path(&a.path);

            let b_version = best_version_in_path(&b.path);

            match (a_version, b_version) {
                (Some(a_version), Some(b_version)) => b_version
                    .cmp(&a_version)
                    .then_with(|| default_keep_order(a, b)),

                (Some(_), None) => std::cmp::Ordering::Less,

                (None, Some(_)) => std::cmp::Ordering::Greater,

                (None, None) => default_keep_order(a, b),
            }
        }

        KeepPolicy::Default => default_keep_order(a, b),
    }
}

fn preferred_path_rank(path: &Path, prefer_paths: &[PathBuf]) -> Option<usize> {
    if prefer_paths.is_empty() {
        return None;
    }

    let canonical_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    prefer_paths
        .iter()
        .enumerate()
        .find_map(|(index, preferred_path)| {
            let canonical_preferred =
                fs::canonicalize(preferred_path).unwrap_or_else(|_| preferred_path.clone());

            if canonical_path.starts_with(canonical_preferred) {
                Some(index)
            } else {
                None
            }
        })
}

fn default_keep_order(a: &CandidateFile, b: &CandidateFile) -> std::cmp::Ordering {
    keep_score(&a.path)
        .cmp(&keep_score(&b.path))
        .then_with(|| {
            a.path
                .components()
                .count()
                .cmp(&b.path.components().count())
        })
        .then_with(|| a.path.to_string_lossy().cmp(&b.path.to_string_lossy()))
}

fn best_version_in_path(path: &Path) -> Option<Vec<u64>> {
    path.components()
        .filter_map(|component| {
            component
                .as_os_str()
                .to_str()
                .and_then(parse_version_component)
        })
        .max()
}

fn parse_version_component(value: &str) -> Option<Vec<u64>> {
    let trimmed = value.trim_start_matches('v').trim_start_matches('V');

    if !looks_like_version_component(trimmed) {
        return None;
    }

    let mut numbers = Vec::new();

    for part in trimmed.split('.') {
        let Ok(number) = part.parse::<u64>() else {
            return None;
        };

        numbers.push(number);
    }

    if numbers.is_empty() {
        None
    } else {
        Some(numbers)
    }
}

fn looks_like_version_component(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    let mut dot_count = 0usize;

    for character in value.chars() {
        if character == '.' {
            dot_count += 1;
            continue;
        }

        if !character.is_ascii_digit() {
            return false;
        }
    }

    dot_count >= 1
}

fn keep_score(path: &Path) -> u8 {
    let text = path.to_string_lossy();

    if text.contains("/Downloads/") || text.ends_with("/Downloads") {
        return 3;
    }

    if text.contains("/.cache/")
        || text.contains("/.cargo/")
        || text.contains("/.gradle/")
        || text.contains("/.local/share/")
    {
        return 2;
    }

    0
}
