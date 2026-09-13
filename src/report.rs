// SPDX-License-Identifier: BSD-3-Clause

use std::env;
use std::ffi::CStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::classify::classify_group;
use crate::model::{DuplicateGroup, KeepPolicy, Options, PathClass, QuarantineSummary};
use crate::output::human_size;

const GROUP_SEPARATOR: &str =
    "--------------------------------------------------------------------------------";

struct SystemInfo {
    os: String,
    arch: String,
    hostname: String,
}

fn system_info() -> SystemInfo {
    let mut uts = MaybeUninit::<libc::utsname>::uninit();

    let result = unsafe { libc::uname(uts.as_mut_ptr()) };

    if result != 0 {
        return SystemInfo {
            os: "unknown".to_string(),
            arch: std::env::consts::ARCH.to_string(),
            hostname: "unknown".to_string(),
        };
    }

    let uts = unsafe { uts.assume_init() };

    let sysname = unsafe { CStr::from_ptr(uts.sysname.as_ptr()) }.to_string_lossy();

    let release = unsafe { CStr::from_ptr(uts.release.as_ptr()) }.to_string_lossy();

    let machine = unsafe { CStr::from_ptr(uts.machine.as_ptr()) }.to_string_lossy();

    let nodename = unsafe { CStr::from_ptr(uts.nodename.as_ptr()) }.to_string_lossy();

    SystemInfo {
        os: format!("{}-{}", sysname, release),
        arch: machine.into_owned(),
        hostname: nodename.into_owned(),
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {:02}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

fn format_system_time(time: SystemTime) -> String {
    let seconds = time
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as libc::time_t;

    let mut local_tm = MaybeUninit::<libc::tm>::uninit();

    let result = unsafe { libc::localtime_r(&seconds, local_tm.as_mut_ptr()) };

    if result.is_null() {
        return "unknown".to_string();
    }

    let local_tm = unsafe { local_tm.assume_init() };

    let format = b"%Y-%m-%d %H:%M:%S\0";

    let mut buffer = [0u8; 32];

    let written = unsafe {
        libc::strftime(
            buffer.as_mut_ptr() as *mut libc::c_char,
            buffer.len(),
            format.as_ptr() as *const libc::c_char,
            &local_tm,
        )
    };

    if written == 0 {
        return "unknown".to_string();
    }

    String::from_utf8_lossy(&buffer[..written]).into_owned()
}

pub fn write_duplicate_report(
    duplicate_groups: &[DuplicateGroup],
    options: &Options,
    scan_date: SystemTime,
    scan_duration: Duration,
    quarantine_summary: Option<&QuarantineSummary>,
) -> io::Result<PathBuf> {
    let report_path = resolve_report_path(options)?;

    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&report_path)?;

    let system = system_info();

    writeln!(file, "Nightkrawler duplicate report")?;
    writeln!(file)?;

    writeln!(file, "Date: {}", format_system_time(scan_date))?;
    writeln!(file, "OS: {}", system.os)?;
    writeln!(file, "Arch: {}", system.arch)?;
    writeln!(file, "Hostname: {}", system.hostname)?;
    writeln!(file, "Total scan: {}", format_duration(scan_duration))?;
    writeln!(file, "Scan root: {}", options.root_path.display())?;
    writeln!(file, "Minimum size: {}", human_size(options.min_size))?;
    writeln!(
        file,
        "Recursive: {}",
        if options.recursive { "yes" } else { "no" }
    )?;
    writeln!(file, "Mode: {}", report_mode_label(options))?;
    writeln!(
        file,
        "Keep policy: {}",
        keep_policy_label(options.keep_policy)
    )?;
    writeln!(file)?;

    writeln!(file, "Exclude:")?;

    if options.excludes.is_empty() {
        writeln!(file, "    none")?;
    } else {
        for path in &options.excludes {
            writeln!(file, "    {}", path.display())?;
        }
    }

    writeln!(file)?;

    writeln!(file, "Protect:")?;

    if options.protected_paths.is_empty() {
        writeln!(file, "    none")?;
    } else {
        for path in &options.protected_paths {
            writeln!(file, "    {}", path.display())?;
        }
    }

    writeln!(file)?;

    if duplicate_groups.is_empty() {
        writeln!(file, "No duplicate files found.")?;
        file.flush()?;

        return Ok(report_path);
    }

    let action_label = report_action_label(options);

    let mut files_removable = 0usize;
    let mut reclaimable_space = 0u64;

    let mut group_number = 1usize;

    for class in [
        PathClass::Protected,
        PathClass::Generated,
        PathClass::Ordinary,
    ] {
        let groups: Vec<&DuplicateGroup> = duplicate_groups
            .iter()
            .filter(|group| classify_group(group, &options.protected_paths) == class)
            .collect();

        if groups.is_empty() {
            continue;
        }

        writeln!(file, "Section: {}", class_section_name(class))?;
        writeln!(file)?;

        for group in groups {
            writeln!(file, "{}", GROUP_SEPARATOR)?;
            writeln!(file)?;

            writeln!(file, "Duplicate group {}", group_number)?;
            writeln!(file, "  Class: {}", class_label(class))?;
            writeln!(file, "  Size: {}", human_size(group.keep.size))?;
            writeln!(file)?;

            writeln!(file, "  To Keep:")?;
            writeln!(file, "    {}", group.keep.path.display())?;
            writeln!(file)?;

            writeln!(file, "  {}:", action_label)?;

            for candidate in &group.remove {
                writeln!(file, "    {}", candidate.path.display())?;

                files_removable += 1;
                reclaimable_space = reclaimable_space.saturating_add(candidate.size);
            }

            writeln!(file)?;

            group_number += 1;
        }

        writeln!(file, "{}", GROUP_SEPARATOR)?;
        writeln!(file)?;
    }

    writeln!(file, "Summary")?;
    writeln!(file, "  Duplicate groups: {}", duplicate_groups.len())?;
    writeln!(file, "  Files kept: {}", duplicate_groups.len())?;
    writeln!(file, "  Files removable: {}", files_removable)?;
    writeln!(
        file,
        "  Reclaimable space: {}",
        human_size(reclaimable_space)
    )?;

    if let (Some(quarantine_path), Some(summary)) =
        (options.quarantine_path.as_ref(), quarantine_summary)
    {
        writeln!(file)?;
        writeln!(file, "Quarantine")?;
        writeln!(file, "  Files: {}", summary.copied)?;
        writeln!(file, "  Location: {}", quarantine_path.display())?;
    }

    file.flush()?;

    Ok(report_path)
}

fn report_mode_label(options: &Options) -> &'static str {
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

fn keep_policy_label(policy: KeepPolicy) -> &'static str {
    match policy {
        KeepPolicy::Default => "default",
        KeepPolicy::HighestVersion => "highest version",
    }
}

fn resolve_report_path(options: &Options) -> io::Result<PathBuf> {
    let Some(path) = options.report_path.as_ref() else {
        return Ok(env::current_dir()?.join(report_filename()));
    };

    if path.is_dir() {
        return Ok(path.join(report_filename()));
    }

    Ok(path.clone())
}

fn report_filename() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!("nightkrawler-duplicates-{}.txt", timestamp)
}

fn class_section_name(class: PathClass) -> &'static str {
    match class {
        PathClass::Protected => "Protected",
        PathClass::Generated => "Generated",
        PathClass::Ordinary => "Ordinary",
    }
}

fn class_label(class: PathClass) -> &'static str {
    match class {
        PathClass::Protected => "protected",
        PathClass::Generated => "generated",
        PathClass::Ordinary => "ordinary",
    }
}

fn report_action_label(options: &Options) -> &'static str {
    if options.quarantine_path.is_some() {
        "To Quarantine"
    } else if options.trash {
        "To Trash"
    } else if options.delete {
        "To Delete"
    } else {
        "Removable"
    }
}
