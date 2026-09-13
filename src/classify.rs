// SPDX-License-Identifier: BSD-3-Clause

use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::model::{DuplicateGroup, PathClass};

const GENERATED_DIRECTORY_NAMES: &[&str] = &[
    "target",
    "build",
    "dist",
    ".cache",
    "__pycache__",
    "node_modules",
    ".gradle",
];

pub fn classify_path(path: &Path, protected_paths: &[PathBuf]) -> PathClass {
    if matching_protected_root(path, protected_paths).is_some() {
        PathClass::Protected
    } else if is_generated_path(path) {
        PathClass::Generated
    } else {
        PathClass::Ordinary
    }
}

pub fn classify_group(group: &DuplicateGroup, protected_paths: &[PathBuf]) -> PathClass {
    std::iter::once(&group.keep)
        .chain(group.remove.iter())
        .map(|file| classify_path(&file.path, protected_paths))
        .max()
        .unwrap_or(PathClass::Ordinary)
}

pub fn matching_protected_root(path: &Path, protected_paths: &[PathBuf]) -> Option<PathBuf> {
    let canonical_path = canonical_or_original(path);

    protected_roots(protected_paths)
        .into_iter()
        .filter(|root| canonical_path.starts_with(root))
        .max_by_key(|root| root.components().count())
}

fn is_generated_path(path: &Path) -> bool {
    path.components().any(|component| {
        let Component::Normal(name) = component else {
            return false;
        };

        let Some(name) = name.to_str() else {
            return false;
        };

        GENERATED_DIRECTORY_NAMES.contains(&name)
    })
}

fn protected_roots(protected_paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut roots = vec![
        PathBuf::from("/boot"),
        PathBuf::from("/etc"),
        PathBuf::from("/root"),
    ];

    #[cfg(target_os = "freebsd")]
    roots.push(PathBuf::from("/usr/local/etc"));

    #[cfg(target_os = "netbsd")]
    roots.push(PathBuf::from("/usr/pkg/etc"));

    if let Some(home) = env::var_os("HOME") {
        let home = PathBuf::from(home);

        roots.push(home.join(".ssh"));
        roots.push(home.join(".gnupg"));
        roots.push(home.join(".config"));
    }

    roots.extend(protected_paths.iter().cloned());

    roots
        .into_iter()
        .map(|path| canonical_or_original(&path))
        .collect()
}

fn canonical_or_original(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}
