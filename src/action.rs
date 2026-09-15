// SPDX-License-Identifier: BSD-3-Clause

use crate::human_size;
use crate::model::{CandidateFile, DeleteSummary, DuplicateGroup, TrashSummary};
use crate::output::{
    COLOR_ERROR, COLOR_HEADING, COLOR_MUTED, COLOR_PATH, COLOR_VALUE, COLOR_WARNING, colored,
    print_section,
};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn delete_duplicates(
    duplicate_groups: &[DuplicateGroup],
    interactive: bool,
    use_colors: bool,
) -> DeleteSummary {
    if duplicate_groups.is_empty() {
        return DeleteSummary::default();
    }

    let kept = duplicate_groups.len();
    let mut deleted = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;
    let mut deleted_space = 0u64;

    for group in duplicate_groups {
        for file in &group.remove {
            if interactive && !confirm_delete_file(file) {
                println!("{}", colored("Skipped:", COLOR_HEADING, use_colors,),);

                println!(
                    "  {}",
                    colored(file.path.display(), COLOR_MUTED, use_colors,),
                );

                println!();

                skipped += 1;

                continue;
            }

            match fs::remove_file(&file.path) {
                Ok(()) => {
                    if interactive {
                        println!("{}", colored("Deleted:", COLOR_MUTED, use_colors,),);

                        println!(
                            "  {}",
                            colored(file.path.display(), COLOR_ERROR, use_colors,),
                        );

                        println!();
                    }

                    deleted += 1;
                    deleted_space += file.size;
                }

                Err(error) => {
                    eprintln!(
                        "{}",
                        colored(
                            format!(
                                "nightkrawler: unable to delete {}: {}",
                                file.path.display(),
                                error
                            ),
                            COLOR_ERROR,
                            use_colors,
                        ),
                    );

                    failed += 1;
                }
            }
        }
    }

    DeleteSummary {
        kept,
        deleted,
        failed,
        skipped,
        deleted_space,
    }
}

pub fn verify_trash_available(duplicate_groups: &[DuplicateGroup]) -> Result<(), String> {
    let mut checked_filesystems = HashSet::new();

    for group in duplicate_groups {
        for file in &group.remove {
            let metadata = fs::metadata(&file.path).map_err(|error| {
                format!(
                    "unable to inspect {} before Trash preflight: {}",
                    file.path.display(),
                    error
                )
            })?;

            let device = metadata.dev();

            if !checked_filesystems.insert(device) {
                continue;
            }

            let Some(parent) = file.path.parent() else {
                return Err(format!(
                    "unable to determine parent directory for {}",
                    file.path.display()
                ));
            };

            verify_trash_for_directory(parent)?;
        }
    }

    Ok(())
}

fn verify_trash_for_directory(directory: &Path) -> Result<(), String> {
    let probe_path = create_trash_probe(directory)?;

    match trash::delete(&probe_path) {
        Ok(()) => Ok(()),

        Err(error) => {
            let _ = fs::remove_file(&probe_path);

            Err(format!(
                "Trash is unavailable for filesystem containing {}: {}",
                directory.display(),
                error
            ))
        }
    }
}

fn create_trash_probe(directory: &Path) -> Result<PathBuf, String> {
    for counter in 0..1000usize {
        let probe_path = directory.join(format!(
            ".nightkrawler-trash-probe-{}-{}",
            std::process::id(),
            counter
        ));

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&probe_path)
        {
            Ok(_) => {
                return Ok(probe_path);
            }

            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                continue;
            }

            Err(error) => {
                return Err(format!(
                    "unable to create Trash preflight file in {}: {}",
                    directory.display(),
                    error
                ));
            }
        }
    }

    Err(format!(
        "unable to create unique Trash preflight file in {}",
        directory.display()
    ))
}

pub fn trash_duplicates(
    duplicate_groups: &[DuplicateGroup],
    interactive: bool,
    use_colors: bool,
) -> TrashSummary {
    if duplicate_groups.is_empty() {
        return TrashSummary::default();
    }

    print_section("Trash", use_colors);

    println!();

    let kept = duplicate_groups.len();
    let mut trashed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;
    let mut trashed_space = 0u64;

    'groups: for group in duplicate_groups {
        for file in &group.remove {
            if interactive && !confirm_trash_file(file) {
                println!("{}", colored("Skipped:", COLOR_HEADING, use_colors,),);

                println!(
                    "  {}",
                    colored(file.path.display(), COLOR_MUTED, use_colors,),
                );

                println!();

                skipped += 1;

                continue;
            }

            match trash::delete(&file.path) {
                Ok(()) => {
                    println!("{}", colored("Trashed:", COLOR_HEADING, use_colors,),);

                    println!(
                        "  {}",
                        colored(file.path.display(), COLOR_VALUE, use_colors,),
                    );

                    println!();

                    trashed += 1;
                    trashed_space += file.size;
                }

                Err(error) => {
                    eprintln!(
                        "{}",
                        colored(
                            format!(
                                "nightkrawler: unable to move {} to Trash: {}",
                                file.path.display(),
                                error
                            ),
                            COLOR_ERROR,
                            use_colors,
                        ),
                    );

                    failed += 1;

                    eprintln!();

                    eprintln!(
                        "{}",
                        colored(
                            "Trash operation stopped after the first failure.",
                            COLOR_ERROR,
                            use_colors,
                        ),
                    );

                    break 'groups;
                }
            }
        }
    }

    TrashSummary {
        kept,
        trashed,
        failed,
        skipped,
        trashed_space,
    }
}

fn confirm_delete_file(file: &CandidateFile) -> bool {
    println!(
        "{}",
        colored("Candidate for deletion:", COLOR_HEADING, true,),
    );

    println!(
        "  {}: {}",
        colored("path", COLOR_WARNING, true),
        colored(file.path.display(), COLOR_PATH, true),
    );

    println!(
        "  {}: {}",
        colored("size", COLOR_MUTED, true),
        colored(human_size(file.size), COLOR_VALUE, true),
    );

    println!();

    ask_yes_no_default_no(&colored("Delete this file? [y/N] ", COLOR_WARNING, true))
}

fn confirm_trash_file(file: &CandidateFile) -> bool {
    println!("{}", colored("Candidate for Trash:", COLOR_HEADING, true,),);

    println!(
        "  {}: {}",
        colored("path", COLOR_MUTED, true),
        colored(file.path.display(), COLOR_WARNING, true),
    );

    println!(
        "  {}: {}",
        colored("size", COLOR_MUTED, true),
        colored(human_size(file.size), COLOR_VALUE, true),
    );

    println!();

    ask_yes_no_default_no(&colored(
        "Move this file to Trash? [y/N] ",
        COLOR_HEADING,
        true,
    ))
}

pub fn ask_yes_no_default_no(prompt: &str) -> bool {
    print!("{}", prompt);

    if let Err(error) = io::stdout().flush() {
        eprintln!("nightkrawler: unable to flush prompt: {}", error);

        return false;
    }

    let mut answer = String::new();

    if let Err(error) = io::stdin().read_line(&mut answer) {
        eprintln!("nightkrawler: unable to read answer: {}", error);

        return false;
    }

    matches!(answer.trim().to_lowercase().as_str(), "y" | "yes")
}
