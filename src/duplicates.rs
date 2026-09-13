// SPDX-License-Identifier: BSD-3-Clause

use crate::keep::duplicate_group_from_files;
use crate::model::{CandidateFile, DuplicateGroup, KeepPolicy};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub fn find_duplicate_groups(
    files: Vec<CandidateFile>,
    keep_policy: KeepPolicy,
    prefer_paths: &[PathBuf],
) -> Vec<DuplicateGroup> {
    let mut by_size: BTreeMap<u64, Vec<CandidateFile>> = BTreeMap::new();

    for file in files {
        by_size.entry(file.size).or_default().push(file);
    }

    let mut duplicate_groups = Vec::new();

    for (_size, same_size_files) in by_size {
        if same_size_files.len() < 2 {
            continue;
        }

        let mut by_hash: HashMap<u64, Vec<CandidateFile>> = HashMap::new();

        for file in same_size_files {
            let Some(hash) = hash_file(&file.path) else {
                continue;
            };

            by_hash.entry(hash).or_default().push(file);
        }

        for (_hash, same_hash_files) in by_hash {
            if same_hash_files.len() < 2 {
                continue;
            }

            let verified_groups = verify_duplicate_candidates(same_hash_files);

            for group in verified_groups {
                if group.len() < 2 {
                    continue;
                }

                let duplicate_group = duplicate_group_from_files(group, keep_policy, prefer_paths);

                duplicate_groups.push(duplicate_group);
            }
        }
    }

    duplicate_groups
}

fn hash_file(path: &Path) -> Option<u64> {
    let file = fs::File::open(path).ok()?;

    let mut reader = BufReader::new(file);

    let mut hasher = DefaultHasher::new();

    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer).ok()?;

        if bytes_read == 0 {
            break;
        }

        buffer[..bytes_read].hash(&mut hasher);
    }

    Some(hasher.finish())
}

fn verify_duplicate_candidates(candidates: Vec<CandidateFile>) -> Vec<Vec<CandidateFile>> {
    let mut groups: Vec<Vec<CandidateFile>> = Vec::new();

    'candidate_loop: for candidate in candidates {
        for group in &mut groups {
            let Some(first_file) = group.first() else {
                continue;
            };

            if files_are_equal(&candidate.path, &first_file.path) {
                group.push(candidate);

                continue 'candidate_loop;
            }
        }

        groups.push(vec![candidate]);
    }

    groups
}

fn files_are_equal(left: &Path, right: &Path) -> bool {
    let left_file = match fs::File::open(left) {
        Ok(file) => file,

        Err(_) => {
            return false;
        }
    };

    let right_file = match fs::File::open(right) {
        Ok(file) => file,

        Err(_) => {
            return false;
        }
    };

    let mut left_reader = BufReader::new(left_file);

    let mut right_reader = BufReader::new(right_file);

    let mut left_buffer = [0u8; 8192];

    let mut right_buffer = [0u8; 8192];

    loop {
        let left_read = match left_reader.read(&mut left_buffer) {
            Ok(bytes_read) => bytes_read,

            Err(_) => {
                return false;
            }
        };

        let right_read = match right_reader.read(&mut right_buffer) {
            Ok(bytes_read) => bytes_read,

            Err(_) => {
                return false;
            }
        };

        if left_read != right_read {
            return false;
        }

        if left_read == 0 {
            return true;
        }

        if left_buffer[..left_read] != right_buffer[..right_read] {
            return false;
        }
    }
}
