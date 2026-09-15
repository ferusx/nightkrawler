// SPDX-License-Identifier: BSD-3-Clause

use crate::model::{
    DeleteSummary, DuplicateGroup, Options, QuarantineSummary, RestoreSummary, TrashSummary,
};

pub const RESET: &str = "\x1b[0m";

pub const COLOR_HEADING: &str = "\x1b[92m"; // green
pub const COLOR_PATH: &str = "\x1b[1;97m"; // Bold bright white
pub const COLOR_SIZE: &str = "\x1b[1;97m"; // Bold bright white

pub const COLOR_WARNING: &str = "\x1b[1;93m"; // bold yellow
pub const COLOR_ERROR: &str = "\x1b[1;31m"; // bold red

pub const COLOR_MUTED: &str = "\x1b[32m"; // green
pub const COLOR_LINE: &str = "\x1b[32m"; // Separator
pub const COLOR_SUCCESS: &str = "\x1b[1;92m"; // bold green
pub const COLOR_VALUE: &str = "\x1b[1;97m"; // bold bright white
pub const COLOR_WORDINGS: &str = "\x1b[37m"; // light gray
pub const COLOR_BRIGHT_YELLOW: &str = "\x1b[1;93m"; // bold bright yellow

pub fn colored(value: impl std::fmt::Display, color: &str, use_colors: bool) -> String {
    if use_colors {
        format!("{}{}{}", color, value, RESET,)
    } else {
        value.to_string()
    }
}

fn print_nested_value(value: impl std::fmt::Display, color: &str, use_colors: bool) {
    println!("    {}", colored(value, color, use_colors,),);
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn scan_mode_label(options: &Options) -> &'static str {
    if options.quarantine_path.is_some() && options.delete {
        "quarantine + permanent delete"
    } else if options.quarantine_path.is_some() && options.trash {
        "quarantine + trash"
    } else if options.quarantine_path.is_some() {
        "quarantine"
    } else if options.delete {
        "permanent delete"
    } else if options.trash {
        "trash"
    } else {
        "report only"
    }
}

pub fn print_report_path(report_path: &std::path::Path, use_colors: bool) {
    println!();

    println!("{}:", colored("Full report", COLOR_MUTED, use_colors),);

    println!(
        "    {}",
        colored(report_path.display(), COLOR_PATH, use_colors,),
    );
}

pub fn print_report(duplicate_groups: &[DuplicateGroup], use_colors: bool) {
    let files_kept = duplicate_groups.len();

    let files_removable: usize = duplicate_groups
        .iter()
        .map(|group| group.remove.len())
        .sum();

    let reclaimable_space: u64 = duplicate_groups
        .iter()
        .flat_map(|group| group.remove.iter())
        .map(|file| file.size)
        .sum();

    println!("{}", colored("─".repeat(88), COLOR_LINE, use_colors,),);

    print_section("Scan summary", use_colors);

    println!(
        "  {}: {},  {}: {},  {}: {},  {}: {}",
        colored("Duplicate groups", COLOR_MUTED, use_colors),
        colored(duplicate_groups.len(), COLOR_VALUE, use_colors),
        colored("Files kept", COLOR_MUTED, use_colors),
        colored(files_kept, COLOR_VALUE, use_colors),
        colored("Files removable", COLOR_MUTED, use_colors),
        colored(files_removable, COLOR_ERROR, use_colors),
        colored("Reclaimable space", COLOR_MUTED, use_colors),
        colored(human_size(reclaimable_space), COLOR_VALUE, use_colors),
    );

    println!("{}", colored("─".repeat(88), COLOR_LINE, use_colors,),);

    println!();
}

pub fn print_restore_summary(summary: &RestoreSummary, use_colors: bool) {
    print_section("Restore summary", use_colors);

    println!(
        "  {}: {},  {}: {},  {}: {},  {}: {}",
        colored("Restored", COLOR_MUTED, use_colors),
        colored(
            summary.restored,
            if summary.restored == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_SUCCESS
            },
            use_colors,
        ),
        colored("Failed", COLOR_MUTED, use_colors),
        colored(
            summary.failed,
            if summary.failed == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_ERROR
            },
            use_colors,
        ),
        colored("Skipped", COLOR_MUTED, use_colors),
        colored(
            summary.skipped,
            if summary.skipped == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_WARNING
            },
            use_colors,
        ),
        colored("Restored space", COLOR_MUTED, use_colors),
        colored(human_size(summary.restored_space), COLOR_SIZE, use_colors,),
    );
}

pub fn print_delete_summary(summary: &DeleteSummary, use_colors: bool) {
    println!("{}", colored("─".repeat(64), COLOR_LINE, use_colors,),);

    print_section("Delete summary", use_colors);

    println!(
        "  {}: {},  {}: {},  {}: {},  {}: {},  {}: {}",
        colored("Kept", COLOR_MUTED, use_colors),
        colored(summary.kept, COLOR_VALUE, use_colors),
        colored("Deleted", COLOR_MUTED, use_colors),
        colored(
            summary.deleted,
            if summary.deleted == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_ERROR
            },
            use_colors,
        ),
        colored("Failed", COLOR_MUTED, use_colors),
        colored(
            summary.failed,
            if summary.failed == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_ERROR
            },
            use_colors,
        ),
        colored("Skipped", COLOR_MUTED, use_colors),
        colored(
            summary.skipped,
            if summary.skipped == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_VALUE
            },
            use_colors,
        ),
        colored("Deleted space", COLOR_MUTED, use_colors),
        colored(human_size(summary.deleted_space), COLOR_SIZE, use_colors,),
    );

    println!("{}", colored("─".repeat(64), COLOR_LINE, use_colors,),);

    println!();

    println!("{}", colored("Delete completed", COLOR_ERROR, use_colors,),);
}

pub fn print_no_duplicates(use_colors: bool) {
    print_section("Scan complete", use_colors);

    print_nested_value("No duplicate files found.", COLOR_SUCCESS, use_colors);

    println!();

    println!("  {}:", colored("Full report", COLOR_MUTED, use_colors),);

    print_nested_value("Nothing to report", COLOR_VALUE, use_colors);
}

pub fn print_scan_banner(options: &Options) {
    print_section("Nightkrawler scan", options.use_colors);

    println!("  {}:", colored("Root", COLOR_MUTED, options.use_colors),);

    print_nested_value(options.root_path.display(), COLOR_VALUE, options.use_colors);

    println!();

    println!(
        "  {}:",
        colored("Recursive", COLOR_MUTED, options.use_colors),
    );

    print_nested_value(
        yes_no(options.recursive),
        COLOR_WORDINGS,
        options.use_colors,
    );

    println!();

    println!(
        "  {}:",
        colored("Minimum size", COLOR_MUTED, options.use_colors),
    );

    print_nested_value(
        human_size(options.min_size),
        COLOR_VALUE,
        options.use_colors,
    );

    println!();

    println!(
        "  {}:",
        colored("Dot-directories", COLOR_MUTED, options.use_colors),
    );

    print_nested_value(
        if options.force_dotdirs {
            "included"
        } else {
            "skipped"
        },
        COLOR_WORDINGS,
        options.use_colors,
    );

    println!();

    println!("  {}:", colored("Mode", COLOR_MUTED, options.use_colors),);

    print_nested_value(scan_mode_label(options), COLOR_WORDINGS, options.use_colors);

    println!();

    println!(
        "{}",
        colored(
            "Scanning... this could take a while",
            COLOR_VALUE,
            options.use_colors,
        ),
    );

    println!();
}

pub fn print_trash_summary(summary: &TrashSummary, use_colors: bool) {
    println!("{}", colored("─".repeat(64), COLOR_LINE, use_colors,),);

    print_section("Trash summary", use_colors);

    println!(
        "  {}: {},  {}: {},  {}: {},  {}: {},  {}: {}",
        colored("Kept", COLOR_MUTED, use_colors),
        colored(summary.kept, COLOR_VALUE, use_colors),
        colored("Trashed", COLOR_MUTED, use_colors),
        colored(
            summary.trashed,
            if summary.trashed == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_WARNING
            },
            use_colors,
        ),
        colored("Failed", COLOR_MUTED, use_colors),
        colored(
            summary.failed,
            if summary.failed == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_ERROR
            },
            use_colors,
        ),
        colored("Skipped", COLOR_MUTED, use_colors),
        colored(
            summary.skipped,
            if summary.skipped == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_VALUE
            },
            use_colors,
        ),
        colored("Trashed space", COLOR_MUTED, use_colors),
        colored(human_size(summary.trashed_space), COLOR_SIZE, use_colors,),
    );

    println!("{}", colored("─".repeat(64), COLOR_LINE, use_colors,),);

    println!();

    println!(
        "{}",
        colored(
            "Trashed files were handed to the system Trash implementation.",
            COLOR_BRIGHT_YELLOW,
            use_colors,
        ),
    );

    println!(
        "{}",
        colored(
            "If they do not appear immediately, check the Trash location used by this filesystem.",
            COLOR_BRIGHT_YELLOW,
            use_colors,
        ),
    );
}

pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "K", "M", "G", "T"];

    let mut size = bytes as f64;

    let mut unit_index = 0usize;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{}{}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1}{}", size, UNITS[unit_index])
    }
}

pub fn print_section(title: &str, use_colors: bool) {
    println!("{}", colored(title, COLOR_HEADING, use_colors,),);
}

pub fn print_quarantine_safety_stop(action: &str, summary: &QuarantineSummary, use_colors: bool) {
    eprintln!("{}", colored("Safety stop", COLOR_ERROR, use_colors,),);

    eprintln!();

    eprintln!(
        "  {}",
        colored(
            "Quarantine did not complete cleanly.",
            COLOR_WARNING,
            use_colors,
        ),
    );

    eprintln!();

    eprintln!("  {}:", colored("Failed copies", COLOR_VALUE, use_colors,),);

    eprintln!(
        "    {}",
        colored(
            summary.failed,
            if summary.failed == 0 {
                COLOR_MUTED
            } else {
                COLOR_ERROR
            },
            use_colors,
        ),
    );

    eprintln!();

    eprintln!("  {}:", colored("Skipped copies", COLOR_VALUE, use_colors,),);

    eprintln!(
        "    {}",
        colored(
            summary.skipped,
            if summary.skipped == 0 {
                COLOR_WORDINGS
            } else {
                COLOR_WARNING
            },
            use_colors,
        ),
    );

    eprintln!();

    eprintln!(
        "{}",
        colored(format!("{} refused.", action,), COLOR_ERROR, use_colors,),
    );

    eprintln!(
        "{}",
        colored(
            "No original files were changed.",
            COLOR_BRIGHT_YELLOW,
            use_colors,
        ),
    );
}
