// SPDX-License-Identifier: BSD-3-Clause

mod action;
mod args;
mod classify;
mod cleanup;
mod duplicates;
mod fsutil;
mod keep;
mod manual;
mod model;
mod output;
mod quarantine;
mod report;
mod restore;
mod scan;
mod validation;

use crate::output::{COLOR_BRIGHT_YELLOW, COLOR_HEADING, COLOR_VALUE, COLOR_WORDINGS};
use action::{delete_duplicates, trash_duplicates, verify_trash_available};
use duplicates::find_duplicate_groups;
use fsutil::available_space_for_path;
use model::{CandidateFile, Command, DuplicateGroup, QuarantineRecord, QuarantineSummary};
use output::{
    COLOR_ERROR, COLOR_MUTED, COLOR_PATH, COLOR_SIZE, COLOR_SUCCESS, COLOR_WARNING, colored,
    human_size, print_delete_summary, print_no_duplicates, print_quarantine_safety_stop,
    print_report, print_report_path, print_scan_banner, print_trash_summary,
};
use quarantine::quarantine_was_complete;
use report::write_duplicate_report;
use scan::collect_regular_files;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    match args::parse_arguments(&arguments) {
        Command::Scan(options) => run_scan(options),

        Command::Restore(quarantine_path) => {
            run_restore(&quarantine_path);
        }

        Command::Cleanup(quarantine_path) => {
            run_cleanup(&quarantine_path);
        }
    }
}

fn run_scan(options: model::Options) {
    print_scan_banner(&options);

    let scan_date = std::time::SystemTime::now();
    let scan_started = std::time::Instant::now();

    let mut files = Vec::new();

    collect_regular_files(&options.root_path, &options, &mut files);

    let duplicate_groups = find_duplicate_groups(files, options.keep_policy, &options.prefer_paths);

    if duplicate_groups.is_empty() {
        print_no_duplicates(options.use_colors);

        return;
    }

    let scan_duration = scan_started.elapsed();

    print_report(&duplicate_groups, options.use_colors);

    if options.delete || options.trash || options.quarantine_path.is_some() {
        let action_name = if options.delete && options.quarantine_path.is_some() {
            "delete/quarantine"
        } else if options.trash && options.quarantine_path.is_some() {
            "trash/quarantine"
        } else if options.delete {
            "delete"
        } else if options.trash {
            "trash"
        } else {
            "quarantine"
        };

        refuse_protected_candidates_unless_allowed(&duplicate_groups, &options, action_name);
    }

    if options.trash {
        if let Err(error) = verify_trash_available(&duplicate_groups) {
            eprintln!(
                "{}",
                colored(
                    format!("nightkrawler: Trash preflight failed: {}", error),
                    COLOR_ERROR,
                    options.use_colors,
                ),
            );

            eprintln!();

            eprintln!(
                "{}",
                colored(
                    "No duplicate files were moved to Trash.",
                    COLOR_BRIGHT_YELLOW,
                    options.use_colors,
                ),
            );

            std::process::exit(1);
        }
    }


    let mut quarantine_summary: Option<QuarantineSummary> = None;

    let mut action_failed = false;

    if let Some(quarantine_path) = options.quarantine_path.as_ref() {
        let quarantine_count: usize = duplicate_groups
            .iter()
            .map(|group| group.remove.len())
            .sum();

        println!(
            "{} {} {}:",
            colored("Quarantining", COLOR_MUTED, options.use_colors),
            colored(quarantine_count, COLOR_VALUE, options.use_colors,),
            colored("files to", COLOR_MUTED, options.use_colors),
        );

        println!(
            "    {}",
            colored(quarantine_path.display(), COLOR_PATH, options.use_colors,),
        );

        let summary = quarantine_duplicates(
            &duplicate_groups,
            &options.root_path,
            quarantine_path,
            options.use_colors,
        );

        if summary.failed == 0 && summary.skipped == 0 {
            if let Err(error) =
                crate::quarantine::write_quarantine_manifest(quarantine_path, &summary.records)
            {
                eprintln!(
                    "nightkrawler: unable to write quarantine manifest: {}",
                    error
                );

                std::process::exit(1);
            }
        }

        if summary.failed > 0 {
            action_failed = true;
        }

        quarantine_summary = Some(summary);
    }



    if options.trash {
        if let Some(summary) = quarantine_summary.as_ref() {
            if !quarantine_was_complete(&duplicate_groups, summary) {
                print_quarantine_safety_stop("Trash", summary, options.use_colors);

                std::process::exit(1);
            }

            println!(
                "{}",
                colored(
                    "Quarantine copy completed",
                    COLOR_BRIGHT_YELLOW,
                    options.use_colors,
                ),
            );

            println!(
                "{}",
                colored(
                    if options.interactive {
                        "Original duplicate files will now be offered for Trash individually"
                    } else {
                        "Original duplicate files will now be moved to Trash"
                    },
                    COLOR_WORDINGS,
                    options.use_colors,
                ),
            );

            println!();
        }

        let trash_summary =
            trash_duplicates(&duplicate_groups, options.interactive, options.use_colors);

        print_trash_summary(&trash_summary, options.use_colors);

        if trash_summary.failed > 0 {
            action_failed = true;
        }
    }

    if options.delete {
        if let Some(summary) = quarantine_summary.as_ref() {
            if !quarantine_was_complete(&duplicate_groups, summary) {
                print_quarantine_safety_stop("Delete", summary, options.use_colors);

                std::process::exit(1);
            }

            println!(
                "{}",
                colored(
                    "Quarantine copy completed",
                    COLOR_BRIGHT_YELLOW,
                    options.use_colors,
                ),
            );

            if options.interactive {
                println!(
                    "{}",
                    colored(
                        "Original duplicate files will now be offered for deletion individually",
                        COLOR_ERROR,
                        options.use_colors,
                    ),
                );
            }

            println!();
        }

        let delete_summary =
            delete_duplicates(&duplicate_groups, options.interactive, options.use_colors);

        print_delete_summary(&delete_summary, options.use_colors);

        if delete_summary.failed > 0 {
            action_failed = true;
        }
    }

    let report_path = match write_duplicate_report(
        &duplicate_groups,
        &options,
        scan_date,
        scan_duration,
        quarantine_summary.as_ref(),
    ) {
        Ok(path) => path,

        Err(error) => {
            eprintln!("nightkrawler: unable to write duplicate report: {}", error);

            std::process::exit(1);
        }
    };

    print_report_path(&report_path, options.use_colors);

    if action_failed {
        std::process::exit(1);
    }
}

fn run_restore(quarantine_path: &Path) {
    println!("{}", colored("Nightkrawler restore", COLOR_HEADING, true,),);

    println!();

    println!("  {}:", colored("Quarantine", COLOR_MUTED, true,),);

    println!(
        "    {}",
        colored(quarantine_path.display(), COLOR_PATH, true,),
    );

    println!();

    let summary = match restore::restore_quarantine(quarantine_path) {
        Ok(summary) => summary,

        Err(error) => {
            eprintln!(
                "{}",
                colored(
                    format!("nightkrawler: unable to restore quarantine: {}", error),
                    COLOR_ERROR,
                    true,
                ),
            );

            std::process::exit(1);
        }
    };

    output::print_restore_summary(&summary, true);

    if summary.failed > 0 {
        std::process::exit(1);
    }
}

fn run_cleanup(quarantine_path: &Path) {
    println!("{}", colored("Nightkrawler cleanup", COLOR_HEADING, true,),);

    println!();

    println!("  {}:", colored("Quarantine", COLOR_MUTED, true,),);

    println!(
        "    {}",
        colored(quarantine_path.display(), COLOR_PATH, true,),
    );

    println!();

    match cleanup::cleanup_quarantine(quarantine_path) {
        Ok(cleanup::CleanupOutcome::Cleaned) => {
            println!(
                "{}",
                colored("Quarantine cleanup completed", COLOR_SUCCESS, true,),
            );
        }

        Ok(cleanup::CleanupOutcome::NothingToClean) => {
            println!("{}", colored("Nothing to clean", COLOR_WORDINGS, true,),);
        }

        Err(error) => {
            eprintln!(
                "{}",
                colored(
                    format!("nightkrawler: unable to clean quarantine: {}", error),
                    COLOR_ERROR,
                    true,
                ),
            );

            std::process::exit(1);
        }
    }
}

fn refuse_protected_candidates_unless_allowed(
    duplicate_groups: &[DuplicateGroup],
    options: &model::Options,
    action_name: &str,
) {
    let protected_candidates: Vec<(&CandidateFile, std::path::PathBuf)> = duplicate_groups
        .iter()
        .flat_map(|group| group.remove.iter())
        .filter_map(|file| {
            crate::classify::matching_protected_root(&file.path, &options.protected_paths)
                .map(|protected_root| (file, protected_root))
        })
        .collect();

    if protected_candidates.is_empty() {
        return;
    }

    if options.allow_protected {
        eprintln!(
            "{}",
            colored(
                format!(
                    "WARNING: --allow-protected permits {} on {} protected candidate(s).",
                    action_name,
                    protected_candidates.len(),
                ),
                COLOR_WARNING,
                options.use_colors,
            ),
        );

        eprintln!();

        return;
    }

    let (first_candidate, protected_root) = &protected_candidates[0];

    eprintln!(
        "{}",
        colored("Safety stop", COLOR_ERROR, options.use_colors,),
    );

    eprintln!();

    eprintln!(
        "  {}:",
        colored("Action refused", COLOR_MUTED, options.use_colors,),
    );

    eprintln!(
        "    {}",
        colored(action_name, COLOR_ERROR, options.use_colors,),
    );

    eprintln!();

    eprintln!(
        "  {}:",
        colored("Protected candidate", COLOR_MUTED, options.use_colors,),
    );

    eprintln!(
        "    {}",
        colored(
            first_candidate.path.display(),
            COLOR_VALUE,
            options.use_colors,
        ),
    );

    eprintln!();

    eprintln!(
        "  {}:",
        colored("Protected root", COLOR_MUTED, options.use_colors,),
    );

    eprintln!(
        "    {}",
        colored(protected_root.display(), COLOR_PATH, options.use_colors,),
    );

    eprintln!();

    eprintln!(
        "  {}:",
        colored("Scan path", COLOR_HEADING, options.use_colors,),
    );

    eprintln!(
        "    {}",
        colored(options.root_path.display(), COLOR_PATH, options.use_colors,),
    );

    if protected_candidates.len() > 1 {
        eprintln!();

        eprintln!(
            "  {}:",
            colored("Protected candidates", COLOR_HEADING, options.use_colors,),
        );

        eprintln!(
            "    {}",
            colored(protected_candidates.len(), COLOR_MUTED, options.use_colors,),
        );
    }

    eprintln!();

    eprintln!(
        "{}",
        colored(
            "No files were changed.",
            COLOR_BRIGHT_YELLOW,
            options.use_colors,
        ),
    );

    eprintln!();

    eprintln!(
        "{}",
        colored(
            "Action modes require --allow-protected for protected candidates.",
            COLOR_HEADING,
            options.use_colors,
        ),
    );

    std::process::exit(1);
}

fn quarantine_duplicates(
    duplicate_groups: &[DuplicateGroup],
    root_path: &Path,
    quarantine_path: &Path,
    use_colors: bool,
) -> QuarantineSummary {
    if duplicate_groups.is_empty() {
        return QuarantineSummary::default();
    }

    let required_space = crate::quarantine::total_removable_size(duplicate_groups);

    if let Err(error) = fs::create_dir_all(quarantine_path) {
        eprintln!(
            "{}",
            colored(
                format!(
                    "nightkrawler: unable to create quarantine directory {}: {}",
                    quarantine_path.display(),
                    error
                ),
                COLOR_ERROR,
                use_colors,
            ),
        );

        std::process::exit(1);
    }

    let available_space = match available_space_for_path(quarantine_path) {
        Ok(available_space) => available_space,

        Err(error) => {
            eprintln!(
                "{}",
                colored(
                    format!(
                        "nightkrawler: unable to check free space for {}: {}",
                        quarantine_path.display(),
                        error
                    ),
                    COLOR_ERROR,
                    use_colors,
                ),
            );

            eprintln!();

            eprintln!(
                "{}",
                colored("Nothing was copied.", COLOR_BRIGHT_YELLOW, use_colors,),
            );

            std::process::exit(1);
        }
    };

    if available_space < required_space {
        eprintln!(
            "{}",
            colored(
                "nightkrawler: not enough free space for quarantine",
                COLOR_ERROR,
                use_colors,
            ),
        );

        eprintln!();

        eprintln!("  {}:", colored("Quarantine path", COLOR_MUTED, use_colors),);

        eprintln!(
            "    {}",
            colored(quarantine_path.display(), COLOR_PATH, use_colors,),
        );

        eprintln!();

        eprintln!("  {}:", colored("Required", COLOR_MUTED, use_colors),);

        eprintln!(
            "    {}",
            colored(human_size(required_space), COLOR_SIZE, use_colors,),
        );

        eprintln!();

        eprintln!("  {}:", colored("Available", COLOR_MUTED, use_colors),);

        eprintln!(
            "    {}",
            colored(human_size(available_space), COLOR_SIZE, use_colors,),
        );

        eprintln!();

        eprintln!(
            "{}",
            colored("Nothing was copied.", COLOR_BRIGHT_YELLOW, use_colors,),
        );

        eprintln!();

        eprintln!(
            "{}",
            colored(
                "Choose a quarantine path on a drive with more free space.",
                COLOR_WARNING,
                use_colors,
            ),
        );

        std::process::exit(1);
    }

    let mut copied = 0usize;

    let mut failed = 0usize;

    let skipped = 0usize;

    let mut records = Vec::new();

    for group in duplicate_groups {
        for file in &group.remove {
            match crate::quarantine::copy_file_to_quarantine(file, root_path, quarantine_path) {
                Ok(destination) => {
                    copied += 1;

                    records.push(QuarantineRecord {
                        original_path: file.path.clone(),
                        quarantine_path: destination,
                        size: file.size,
                    });
                }

                Err(error) => {
                    eprintln!(
                        "{}",
                        colored(
                            format!(
                                "nightkrawler: unable to copy {} to quarantine: {}",
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

    println!();

    QuarantineSummary {
        copied,
        failed,
        skipped,
        records,
    }
}
