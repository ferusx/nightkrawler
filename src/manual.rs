// SPDX-License-Identifier: BSD-3-Clause

use std::env;
use std::ffi::CStr;
use std::io::IsTerminal;

const MIN_INNER_WIDTH: usize = 40;
const MAX_INNER_WIDTH: usize = 96;

/*
 * The manual deliberately uses its own fixed palette rather than the
 * currently selected Noct theme.
 */
const MANUAL_ACCENT: &str = "\x1b[36m"; // The ACCENT (bold cyan) color
const MANUAL_TEXT: &str = "\x1b[37m"; // The TEXT (Dark Gray) color
const MANUAL_EMPHASIS: &str = "\x1b[1;97m"; // The EMPHASIS (white)
const MANUAL_TITLE: &str = "\x1b[1;96m"; // The TITLE (Gray)
const MANUAL_EXAMPLE: &str = "\x1b[32m"; // The EXAMPLE (green) color
const MANUAL_OPTION: &str = "\x1b[1;92m"; // The EXAMPLE (bold+green) color

const MANUAL_BORDER: &str = "\x1b[1;30m"; // The BORDER (gray) color
const MANUAL_RESET: &str = "\x1b[0m";

struct ManualColors {
    accent: &'static str,
    text: &'static str,
    emphasis: &'static str,
    title: &'static str,
    example: &'static str,
    option: &'static str,
    border: &'static str,
    reset: &'static str,
}

impl ManualColors {
    fn new(enabled: bool) -> Self {
        if !enabled {
            return Self {
                accent: "",
                text: "",
                emphasis: "",
                title: "",
                example: "",
                option: "",
                border: "",
                reset: "",
            };
        }

        Self {
            accent: MANUAL_ACCENT,
            text: MANUAL_TEXT,
            emphasis: MANUAL_EMPHASIS,
            title: MANUAL_TITLE,
            example: MANUAL_EXAMPLE,
            option: MANUAL_OPTION,
            border: MANUAL_BORDER,
            reset: MANUAL_RESET,
        }
    }
}
struct ManualPart<'a> {
    text: &'a str,
    color: &'a str,
}

/* =======================================================================
                              --help option
======================================================================= */

pub fn print_help(colors_enabled: bool) {
    let inner_width = manual_inner_width();

    let colors = ManualColors::new(colors_enabled);

    print_top_border(inner_width, &colors);

    print_line(
        "nightkrawler - duplicate file finder and removal tool",
        inner_width,
        colors.title,
        &colors,
    );

    print_separator(inner_width, &colors);

    print_blank_line(inner_width, &colors);

    print_line("Usage:", inner_width, colors.title, &colors);

    print_indented_paragraph(
        &[
            ManualPart {
                text: "nk",
                color: colors.option,
            },
            ManualPart {
                text: "PATH",
                color: colors.emphasis,
            },
            ManualPart {
                text: "--min-size",
                color: colors.option,
            },
            ManualPart {
                text: "SIZE",
                color: colors.emphasis,
            },
            ManualPart {
                text: "[options]",
                color: colors.emphasis,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "nightkrawler",
                color: colors.option,
            },
            ManualPart {
                text: "PATH",
                color: colors.emphasis,
            },
            ManualPart {
                text: "--min-size",
                color: colors.option,
            },
            ManualPart {
                text: "SIZE",
                color: colors.emphasis,
            },
            ManualPart {
                text: "[options]",
                color: colors.emphasis,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_separator(inner_width, &colors);

    print_blank_line(inner_width, &colors);

    print_line("Meta:", inner_width, colors.title, &colors);

    print_option_line("-h, --help", "Show this help text", inner_width, &colors);

    print_option_line("-v, --version", "Show version info", inner_width, &colors);

    print_option_line("--manual", "Show extensive manual", inner_width, &colors);

    print_option_line(
        "--classification-help",
        "Show help on classification",
        inner_width,
        &colors,
    );

    print_option_line(
        "--no-colors",
        "Disable colored output",
        inner_width,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_separator(inner_width, &colors);

    print_blank_line(inner_width, &colors);

    print_line("Options:", inner_width, colors.title, &colors);

    print_option_line(
        "--min-size SIZE",
        "Required. Ignore files smaller than SIZE: 100K, 10M, 1G",
        inner_width,
        &colors,
    );

    print_option_line(
        "--exclude, -E PATH",
        "Exclude one or more paths",
        inner_width,
        &colors,
    );

    print_option_line(
        "--protect, -P PATH",
        "Protect one or more paths from action modes",
        inner_width,
        &colors,
    );

    print_option_line(
        "--allow-protected",
        "Allow action modes on protected paths",
        inner_width,
        &colors,
    );

    print_option_line(
        "--prefer PATH...",
        "Prefer keeping duplicate files under these paths",
        inner_width,
        &colors,
    );

    print_option_line(
        "--force-dotdirs",
        "Include dot-directories during recursive scans",
        inner_width,
        &colors,
    );

    print_option_line(
        "--keep-highest-version",
        "Prefer keeping files in highest version-like path",
        inner_width,
        &colors,
    );

    print_option_line("--recursive, -R", "Scan recursively", inner_width, &colors);

    print_option_line(
        "--quarantine, -q PATH",
        "Copy removable duplicate files into PATH",
        inner_width,
        &colors,
    );

    print_option_line(
        "--trash, -t",
        "Move removable duplicate files to system Trash",
        inner_width,
        &colors,
    );

    print_option_line(
        "--delete, -d",
        "Delete removable duplicate files",
        inner_width,
        &colors,
    );

    print_option_line(
        "--interactive, -i",
        "Ask before each destructive action",
        inner_width,
        &colors,
    );

    print_option_line(
        "--restore, -u PATH",
        "Restore files in quarantine; PATH must point to quarantine",
        inner_width,
        &colors,
    );

    print_option_line(
        "--cleanup, -c",
        "Empty quarantine files and remove its directory",
        inner_width,
        &colors,
    );

    print_option_line(
        "--report, -r PATH",
        "Write report to PATH; directories use an automatic filename",
        inner_width,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_separator(inner_width, &colors);

    print_blank_line(inner_width, &colors);

    print_line("Safety:", inner_width, colors.title, &colors);

    print_indented_paragraph(
        &[
            ManualPart {
                text: "Without",
                color: colors.text,
            },
            ManualPart {
                text: "--quarantine, --trash, or --delete,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "this version is report-only.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "--quarantine",
                color: colors.emphasis,
            },
            ManualPart {
                text: "copies files instead of deleting them.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "--trash",
                color: colors.emphasis,
            },
            ManualPart {
                text: "moves files to Trash, if available, for manual recovery through your file manager.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "--delete",
                color: colors.emphasis,
            },
            ManualPart {
                text: "permanently removes files.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "--interactive",
                color: colors.emphasis,
            },
            ManualPart {
                text: "asks before each trash or delete action.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "--prefer",
                color: colors.emphasis,
            },
            ManualPart {
                text: "only changes which duplicate is kept.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_indented_paragraph(
        &[
            ManualPart {
                text: "If",
                color: colors.text,
            },
            ManualPart {
                text: "--quarantine",
                color: colors.emphasis,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "--trash",
                color: colors.emphasis,
            },
            ManualPart {
                text: "are used together, files are copied first.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "If",
                color: colors.text,
            },
            ManualPart {
                text: "--quarantine",
                color: colors.emphasis,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "--delete",
                color: colors.emphasis,
            },
            ManualPart {
                text: "are used together, files are copied first.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "If quarantine copying fails or is skipped, trash or delete is refused.",
            color: colors.text,
        }],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "Nightkrawler checks free space before copying.",
            color: colors.text,
        }],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "It does not move files except when using",
                color: colors.text,
            },
            ManualPart {
                text: "--trash.",
                color: colors.emphasis,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "It does not follow symlinks.",
            color: colors.text,
        }],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "It skips dot-directories unless",
                color: colors.text,
            },
            ManualPart {
                text: "--force-dotdirs",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is used.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "Protected high-level paths refuse action modes unless",
                color: colors.text,
            },
            ManualPart {
                text: "--allow-protected",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is used.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_separator(inner_width, &colors);

    print_blank_line(inner_width, &colors);

    print_line("Examples:", inner_width, colors.title, &colors);

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "~/Downloads",
                color: colors.accent,
            },
            ManualPart {
                text: "for files >=100MB. (Report only):",
                color: colors.text,
            },
        ],
        "nk ~/Downloads --min-size 100M",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "~/Downloads",
                color: colors.accent,
            },
            ManualPart {
                text: "for files >=100MB recursively. (Report only):",
                color: colors.text,
            },
        ],
        "nk ~/Downloads --recursive --min-size 100M",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "~",
                color: colors.accent,
            },
            ManualPart {
                text: "recursively for files >=100MB. Exclude",
                color: colors.text,
            },
            ManualPart {
                text: "~/bin, ~/PycharmProjects,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "~/snap.",
                color: colors.accent,
            },
            ManualPart {
                text: "(Report only):",
                color: colors.text,
            },
        ],
        "nk ~ --recursive --min-size 100M --exclude ~/bin ~/PycharmProjects ~/snap",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "~",
                color: colors.accent,
            },
            ManualPart {
                text: "recursively for files >=100MB and prefer keeping duplicates under",
                color: colors.text,
            },
            ManualPart {
                text: "~/DevX",
                color: colors.accent,
            },
            ManualPart {
                text: "then",
                color: colors.text,
            },
            ManualPart {
                text: "~/Documents.",
                color: colors.accent,
            },
            ManualPart {
                text: "(Report only):",
                color: colors.text,
            },
        ],
        "nk ~ -R --min-size 100M --prefer ~/DevX ~/Documents",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "~",
                color: colors.accent,
            },
            ManualPart {
                text: "recursively for files >=100MB, prefer keeping duplicates under",
                color: colors.text,
            },
            ManualPart {
                text: "~/DevX,",
                color: colors.accent,
            },
            ManualPart {
                text: "and copy removable duplicates to quarantine:",
                color: colors.text,
            },
        ],
        "nk ~ -R --min-size 100M --prefer ~/DevX --quarantine ~/nk-quarantine",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "/",
                color: colors.accent,
            },
            ManualPart {
                text: "recursively for files 100MB or larger. (Report only):",
                color: colors.text,
            },
        ],
        "nk / -R --min-size 100M",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "/",
                color: colors.accent,
            },
            ManualPart {
                text: "recursively for files 100MB or larger, quarantine removable duplicates to",
                color: colors.text,
            },
            ManualPart {
                text: "/tmp/q,",
                color: colors.accent,
            },
            ManualPart {
                text: "and allow action on protected candidates:",
                color: colors.text,
            },
        ],
        "nk / -R --min-size 100M -q /tmp/q --allow-protected",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan the directory recursively, quarantine removable duplicates, then offer each original duplicate for deletion, while protecting",
                color: colors.text,
            },
            ManualPart {
                text: "~/.sensitive.",
                color: colors.accent,
            },
            ManualPart {
                text: "(Files may be deleted!):",
                color: colors.text,
            },
        ],
        "nk ~ --min-size 100M -q /tmp/q --protect ~/.sensitive -d --interactive",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "~/Downloads",
                color: colors.accent,
            },
            ManualPart {
                text: "for files 100MB or larger, then move removable duplicates to Trash:",
                color: colors.text,
            },
        ],
        "nk ~/Downloads --recursive --min-size 100M --trash",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "~/Downloads",
                color: colors.accent,
            },
            ManualPart {
                text: "recursively for files 100MB or larger and offer each removable duplicate for Trash interactively:",
                color: colors.text,
            },
        ],
        "nk ~/Downloads --recursive --min-size 100M -t -i",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Scan the directory recursively, quarantine removable duplicates, then move the original removable duplicates to Trash:",
            color: colors.text,
        }],
        "nk ~/Downloads --recursive --min-size 100M --quarantine ~/nk-quarantine -t",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Scan the directory recursively and permanently delete removable duplicate files larger than 100MB:",
            color: colors.text,
        }],
        "nk ~/Downloads --recursive --min-size 100M --delete",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Scan the directory recursively and offer each removable duplicate for deletion interactively:",
            color: colors.text,
        }],
        "nk ~/Downloads --recursive --min-size 100M -d -i",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Scan the directory recursively, quarantine removable duplicates, then permanently delete the originals:",
            color: colors.text,
        }],
        "nk ~/Downloads --recursive --min-size 100M -q ~/nk-quarantine -d",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Scan the directory recursively, quarantine removable duplicates, then offer each original removable duplicate for deletion:",
            color: colors.text,
        }],
        "nk ~/Downloads --recursive --min-size 100M -q ~/nk-quarantine -d -i",
        inner_width,
        &colors,
    );

    print_line("Note:", inner_width, colors.title, &colors);

    print_indented_paragraph(
        &[
            ManualPart {
                text: "After the command has finished, Nightkrawler generates a report in the current directory or where specified by",
                color: colors.text,
            },
            ManualPart {
                text: "-r, --report",
                color: colors.emphasis,
            },
            ManualPart {
                text: "PATH.",
                color: colors.emphasis,
            },
            ManualPart {
                text: "The report contains detailed information from the scan, quarantine, and Trash/delete process, as well as metadata about the scanned host.",
                color: colors.text,
            },
        ],
        inner_width,
        2,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_bottom_border(inner_width, &colors);
}

/* =======================================================================
                         --manual option
======================================================================= */
pub fn print_manual(colors_enabled: bool) {
    let inner_width = manual_inner_width();

    let colors = ManualColors::new(colors_enabled);

    print_top_border(inner_width, &colors);

    print_line("Nightkrawler Manual", inner_width, colors.title, &colors);

    print_separator(inner_width, &colors);

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "Nightkrawler is a duplicate file finder and removal tool designed around cautious inspection, explicit action modes, and recoverable workflows.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_section("Overview", inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "Nightkrawler scans regular files, groups files with identical content, chooses one file from each duplicate group to keep, and identifies the remaining copies as removable duplicates.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Without",
                color: colors.text,
            },
            ManualPart {
                text: "--quarantine, --trash,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "--delete,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "Nightkrawler operates in report-only mode. It scans and reports duplicate groups without changing the original files.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "A duplicate group always keeps one matching file. Action modes operate only on the removable members of the group.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Nightkrawler compares file contents rather than filenames. Files may have completely different names and still belong to the same duplicate group when their contents are identical.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Zero-byte files follow the same duplicate rule as every other regular file. If several zero-byte files form a duplicate group, one is kept and the others are treated as removable duplicates.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_section("Usage", inner_width, &colors);

    print_indented_paragraph(
        &[
            ManualPart {
                text: "nk",
                color: colors.option,
            },
            ManualPart {
                text: "PATH",
                color: colors.emphasis,
            },
            ManualPart {
                text: "--min-size",
                color: colors.option,
            },
            ManualPart {
                text: "SIZE",
                color: colors.emphasis,
            },
            ManualPart {
                text: "[options]",
                color: colors.emphasis,
            },
        ],
        inner_width,
        4,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "nightkrawler",
                color: colors.option,
            },
            ManualPart {
                text: "PATH",
                color: colors.emphasis,
            },
            ManualPart {
                text: "--min-size",
                color: colors.option,
            },
            ManualPart {
                text: "SIZE",
                color: colors.emphasis,
            },
            ManualPart {
                text: "[options]",
                color: colors.emphasis,
            },
        ],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "PATH",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is the directory or file tree Nightkrawler will inspect. Only one scan path may be supplied.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "--min-size",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is required for scan commands. Files smaller than the selected threshold are ignored before duplicate comparison begins.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "SIZE",
                color: colors.emphasis,
            },
            ManualPart {
                text: "accepts byte values or binary-style suffixes such as",
                color: colors.text,
            },
            ManualPart {
                text: "100K, 10M, 1G,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "1T.",
                color: colors.accent,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "The short command name",
                color: colors.text,
            },
            ManualPart {
                text: "nk",
                color: colors.emphasis,
            },
            ManualPart {
                text: "and the full executable name",
                color: colors.text,
            },
            ManualPart {
                text: "nightkrawler",
                color: colors.emphasis,
            },
            ManualPart {
                text: "refer to the same program when the short command is installed as a symlink or configured as a shell alias. Packaged installations should provide the",
                color: colors.text,
            },
            ManualPart {
                text: "nk",
                color: colors.emphasis,
            },
            ManualPart {
                text: "symlink automatically.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_section("Scanning", inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "Nightkrawler scans regular files only. Symbolic links are not followed and are not added to duplicate groups.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Without",
                color: colors.text,
            },
            ManualPart {
                text: "--recursive",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "-R,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "Nightkrawler inspects files directly under the selected scan path. Recursive mode descends into subdirectories.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Dot-directories are skipped during recursive scanning unless",
                color: colors.text,
            },
            ManualPart {
                text: "--force-dotdirs",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is supplied.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Paths supplied with",
                color: colors.text,
            },
            ManualPart {
                text: "--exclude",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "-E",
                color: colors.emphasis,
            },
            ManualPart {
                text: "are omitted from scanning. Multiple excluded paths may be supplied after the option.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "When a quarantine directory is configured, Nightkrawler also excludes that quarantine tree from the scan so copied quarantine files cannot immediately become duplicate candidates themselves.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Files are first grouped by size. Files with unique sizes require no further duplicate comparison. Matching-size candidates are hashed and then verified by direct byte comparison before they are accepted as identical.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "This final byte comparison prevents a hash match alone from being treated as proof that two files are identical.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_section("Classification", inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Nightkrawler classifies duplicate paths as",
                color: colors.text,
            },
            ManualPart {
                text: "ordinary, generated,",
                color: colors.accent,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "protected.",
                color: colors.accent,
            },
            ManualPart {
                text: "Classification helps distinguish normal files from generated trees and filesystem areas that deserve additional protection.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "Ordinary",
        &[ManualPart {
            text: "An ordinary path is a file that does not fall under a generated or protected path rule.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "Generated",
        &[
            ManualPart {
                text: "Generated paths are recognized when their path contains common generated-directory names such as",
                color: colors.text,
            },
            ManualPart {
                text: "target, build, dist, .cache, __pycache__, node_modules,",
                color: colors.accent,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: ".gradle.",
                color: colors.accent,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Generated classification does not automatically delete anything. It describes where the duplicate group was found and helps make reports easier to evaluate.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "Protected",
        &[
            ManualPart {
                text: "Protected paths are filesystem trees where action modes require explicit permission through",
                color: colors.text,
            },
            ManualPart {
                text: "--allow-protected.",
                color: colors.emphasis,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Built-in protected roots include",
                color: colors.text,
            },
            ManualPart {
                text: "/boot, /etc,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "/root.",
                color: colors.accent,
            },
        ],
        inner_width,
        &colors,
    );

    #[cfg(target_os = "freebsd")]
    print_manual_paragraph(
        &[
            ManualPart {
                text: "On FreeBSD,",
                color: colors.text,
            },
            ManualPart {
                text: "/usr/local/etc",
                color: colors.accent,
            },
            ManualPart {
                text: "is also protected by default.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    #[cfg(target_os = "netbsd")]
    print_manual_paragraph(
        &[
            ManualPart {
                text: "On NetBSD,",
                color: colors.text,
            },
            ManualPart {
                text: "/usr/pkg/etc",
                color: colors.accent,
            },
            ManualPart {
                text: "is also protected by default.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "When",
                color: colors.text,
            },
            ManualPart {
                text: "HOME",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is available, Nightkrawler also protects",
                color: colors.text,
            },
            ManualPart {
                text: "~/.ssh, ~/.gnupg,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "~/.config.",
                color: colors.accent,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Additional protected trees may be supplied with",
                color: colors.text,
            },
            ManualPart {
                text: "--protect",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "-P.",
                color: colors.emphasis,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "If any file in a duplicate group belongs to a more restrictive classification, the group is reported using that classification.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "For a shorter explanation focused only on classification, use",
                color: colors.text,
            },
            ManualPart {
                text: "--classification-help.",
                color: colors.emphasis,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_section("Keeper Selection", inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "Every duplicate group contains one file selected as the keeper. The remaining matching files become removable candidates.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Keeper selection never means that Nightkrawler considers the other files different. All members of the group have already been verified as identical by content.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "Default policy",
        &[ManualPart {
            text: "The default keeper policy uses deterministic path-based ordering. Files under locations commonly associated with temporary or replaceable content are less preferred than ordinary locations.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Examples of less-preferred locations include",
                color: colors.text,
            },
            ManualPart {
                text: "Downloads, .cache, .cargo, .gradle,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: ".local/share.",
                color: colors.accent,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "When candidates remain tied, Nightkrawler uses path depth and pathname ordering to make the final choice reproducible.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "--prefer",
        &[
            ManualPart {
                text: "Use",
                color: colors.text,
            },
            ManualPart {
                text: "--prefer",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "-p",
                color: colors.emphasis,
            },
            ManualPart {
                text: "to give one or more directory trees explicit keeper priority.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Preferred paths are ordered. The first path has the highest priority, followed by the second, and so on.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "nk ~ -R --min-size 100M --prefer ~/DevX ~/Documents",
            color: colors.example,
        }],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "In this example, a matching file under",
                color: colors.text,
            },
            ManualPart {
                text: "~/DevX",
                color: colors.accent,
            },
            ManualPart {
                text: "is preferred first. A matching file under",
                color: colors.text,
            },
            ManualPart {
                text: "~/Documents",
                color: colors.accent,
            },
            ManualPart {
                text: "has the next priority.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "--keep-highest-version",
        &[
            ManualPart {
                text: "The",
                color: colors.text,
            },
            ManualPart {
                text: "--keep-highest-version",
                color: colors.emphasis,
            },
            ManualPart {
                text: "policy searches pathname components for version-like values such as",
                color: colors.text,
            },
            ManualPart {
                text: "1.2, 2.0.1,",
                color: colors.accent,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "v3.4.",
                color: colors.accent,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "When version-like candidates are available, Nightkrawler prefers the file associated with the highest detected version. Other keeper rules are used to break ties or when no usable version is found.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Explicit",
                color: colors.text,
            },
            ManualPart {
                text: "--prefer",
                color: colors.emphasis,
            },
            ManualPart {
                text: "paths take priority before the selected keeper policy is applied.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_section("Protection", inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "Protection is separate from exclusion. An excluded path is not scanned. A protected path may still be scanned and reported, but action modes are prevented from operating on protected candidates unless explicit permission is given.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Use",
                color: colors.text,
            },
            ManualPart {
                text: "--protect",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "-P",
                color: colors.emphasis,
            },
            ManualPart {
                text: "to add one or more protected path trees for the current command.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "nk ~ -R --min-size 100M --protect ~/.sensitive",
            color: colors.example,
        }],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "The protected tree remains visible to classification and reporting. Protection becomes decisive when",
                color: colors.text,
            },
            ManualPart {
                text: "--quarantine, --trash,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "--delete",
                color: colors.emphasis,
            },
            ManualPart {
                text: "would act on a protected removable candidate.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "If a protected removable candidate is found during an action mode, Nightkrawler performs a safety stop unless",
                color: colors.text,
            },
            ManualPart {
                text: "--allow-protected",
                color: colors.emphasis,
            },
            ManualPart {
                text: "was supplied.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "The safety stop identifies the protected candidate, the matching protected root, and the scan path. Original files are not changed by the refused action.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "--allow-protected",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is therefore an explicit override. It should be used only when action on the protected candidates is intentional.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "For ordinary report-only inspection,",
                color: colors.text,
            },
            ManualPart {
                text: "--protect",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is still useful because the classification and generated report will identify matching groups as protected.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_section("Action Modes", inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Nightkrawler has three action modes:",
                color: colors.text,
            },
            ManualPart {
                text: "--quarantine, --trash,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "--delete.",
                color: colors.emphasis,
            },
            ManualPart {
                text: "Without an action mode, a scan is report-only.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "--quarantine",
        &[ManualPart {
            text: "Copies removable duplicates into a quarantine directory while leaving the original files in place.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "--trash",
        &[ManualPart {
            text: "Moves removable duplicates to the system Trash implementation.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "--delete",
        &[ManualPart {
            text: "Permanently removes removable duplicate files from their original locations.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "--trash",
                color: colors.emphasis,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "--delete",
                color: colors.emphasis,
            },
            ManualPart {
                text: "are mutually exclusive. Choose one destructive action for a scan.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "--quarantine",
                color: colors.emphasis,
            },
            ManualPart {
                text: "may be combined with either",
                color: colors.text,
            },
            ManualPart {
                text: "--trash",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "--delete.",
                color: colors.emphasis,
            },
            ManualPart {
                text: "In that workflow, Nightkrawler creates the quarantine copies before changing the originals.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "--interactive",
                color: colors.emphasis,
            },
            ManualPart {
                text: "may be used with",
                color: colors.text,
            },
            ManualPart {
                text: "--trash",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "--delete",
                color: colors.emphasis,
            },
            ManualPart {
                text: "to ask before each destructive action. It does not make quarantine copying interactive.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_section("Quarantine", inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Quarantine is Nightkrawler's recoverable copy stage. Use",
                color: colors.text,
            },
            ManualPart {
                text: "--quarantine",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "-q",
                color: colors.emphasis,
            },
            ManualPart {
                text: "followed by the directory where removable duplicate copies should be stored.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine",
            color: colors.example,
        }],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "Nightkrawler preserves the scanned path structure below the quarantine directory where possible. If a destination name already exists, a unique Nightkrawler suffix is added rather than overwriting the existing quarantine file.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Before copying begins, Nightkrawler calculates the total size of the removable duplicates and checks the available free space on the filesystem containing the quarantine directory.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "If there is not enough free space, quarantine is refused before files are copied.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "A quarantine manifest records the original path, quarantine path, and size of each copied file. The current manifest format stores Unix pathnames as their original bytes rather than relying on tab- or newline-separated text fields.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "This allows valid Unix filenames containing tabs, newlines, spaces, or non-UTF-8 pathname bytes to be represented without using those characters as field separators.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "When quarantine is combined with",
                color: colors.text,
            },
            ManualPart {
                text: "--trash",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "--delete,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "the destructive stage is allowed to proceed only if all intended quarantine copies completed successfully.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "If quarantine copying fails or is incomplete, Nightkrawler performs a safety stop and refuses the Trash or delete stage. Original files are left unchanged by the refused destructive action.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_section("Restore and Cleanup", inner_width, &colors);

    print_manual_subsection(
        "--restore",
        &[ManualPart {
            text: "Restores files recorded in a Nightkrawler quarantine manifest to their original paths.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "nk --restore ~/nk-quarantine",
            color: colors.example,
        }],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "The short form",
                color: colors.text,
            },
            ManualPart {
                text: "-u",
                color: colors.emphasis,
            },
            ManualPart {
                text: "may be used instead of",
                color: colors.text,
            },
            ManualPart {
                text: "--restore.",
                color: colors.emphasis,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "If the original path already exists, that record is skipped rather than overwritten.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "If the original parent directory no longer exists, Nightkrawler recreates the required directory path before restoring the file.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Restore copies files back to their original locations. The quarantine remains available until it is cleaned separately.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "--cleanup",
        &[ManualPart {
            text: "Removes a recognized Nightkrawler quarantine directory after it is no longer needed.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "nk --cleanup ~/nk-quarantine",
            color: colors.example,
        }],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "The short form",
                color: colors.text,
            },
            ManualPart {
                text: "-c",
                color: colors.emphasis,
            },
            ManualPart {
                text: "may be used instead of",
                color: colors.text,
            },
            ManualPart {
                text: "--cleanup.",
                color: colors.emphasis,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Cleanup is deliberately guarded against dangerous targets. Nightkrawler refuses to clean the filesystem root, the user's home directory, or the current working directory.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "A missing or empty quarantine is treated as having nothing to clean rather than as a destructive cleanup operation.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Before removing a populated directory, cleanup verifies that the directory contains a recognized Nightkrawler quarantine manifest.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_section("Reports", inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "When duplicate groups are found, Nightkrawler writes a detailed text report after processing the scan.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Without",
                color: colors.text,
            },
            ManualPart {
                text: "--report",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "-r,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "the report is created in the current directory with an automatically generated filename.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "If",
                color: colors.text,
            },
            ManualPart {
                text: "--report",
                color: colors.emphasis,
            },
            ManualPart {
                text: "points to an existing directory, Nightkrawler places an automatically named report inside that directory. Otherwise, the supplied path is used as the report filename.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Reports include scan date, operating system, architecture, hostname, scan duration, scan root, minimum size, recursive state, action mode, keeper policy, excluded paths, protected paths, duplicate groups, keeper paths, removable candidates, and reclaimable space.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Duplicate groups are divided into",
                color: colors.text,
            },
            ManualPart {
                text: "Protected, Generated,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "Ordinary",
                color: colors.accent,
            },
            ManualPart {
                text: "sections when those classifications are present.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "When quarantine was used, the report also records the number of copied files and the quarantine location.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_section("Safety", inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "Nightkrawler is designed so that inspection and action remain separate. A normal scan reports duplicate candidates without changing files.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "One file from every verified duplicate group is selected as the keeper. Trash and delete operate only on the remaining removable members.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Nightkrawler does not follow symbolic links.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Dot-directories are skipped recursively unless",
                color: colors.text,
            },
            ManualPart {
                text: "--force-dotdirs",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is explicitly supplied.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Protected removable candidates stop action modes unless",
                color: colors.text,
            },
            ManualPart {
                text: "--allow-protected",
                color: colors.emphasis,
            },
            ManualPart {
                text: "was supplied.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Quarantine checks available space before copying. When quarantine precedes Trash or delete, the destructive stage is refused unless the quarantine copy completed cleanly.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "--interactive",
                color: colors.emphasis,
            },
            ManualPart {
                text: "offers each removable candidate individually before Trash or permanent deletion.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Restoration never overwrites an existing original path. Existing destinations are counted as skipped.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "Cleanup contains additional target guards because removing an entire directory tree carries different risks from ordinary duplicate processing.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    // content

    print_manual_section("Examples", inner_width, &colors);

    print_example(
        &[
            ManualPart {
                text: "Report duplicate files 100MB or larger under",
                color: colors.text,
            },
            ManualPart {
                text: "~/Downloads:",
                color: colors.accent,
            },
        ],
        "nk ~/Downloads --min-size 100M",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Scan recursively and exclude selected directory trees:",
            color: colors.text,
        }],
        "nk ~ -R --min-size 100M -E ~/bin ~/PycharmProjects ~/snap",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Prefer keeping matching files under",
                color: colors.text,
            },
            ManualPart {
                text: "~/DevX",
                color: colors.accent,
            },
            ManualPart {
                text: "before",
                color: colors.text,
            },
            ManualPart {
                text: "~/Documents:",
                color: colors.accent,
            },
        ],
        "nk ~ -R --min-size 100M -p ~/DevX ~/Documents",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Create recoverable quarantine copies without changing the originals:",
            color: colors.text,
        }],
        "nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Quarantine removable duplicates first, then offer each original for permanent deletion:",
            color: colors.text,
        }],
        "nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine -d -i",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Restore files from a Nightkrawler quarantine:",
            color: colors.text,
        }],
        "nk --restore ~/nk-quarantine",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Remove a quarantine after it is no longer needed:",
            color: colors.text,
        }],
        "nk --cleanup ~/nk-quarantine",
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "For a compact command reference, use",
                color: colors.text,
            },
            ManualPart {
                text: "--help.",
                color: colors.emphasis,
            },
            ManualPart {
                text: "For focused information about ordinary, generated, and protected classifications, use",
                color: colors.text,
            },
            ManualPart {
                text: "--classification-help.",
                color: colors.emphasis,
            },
        ],
        inner_width,
        &colors,
    );

    print_bottom_border(inner_width, &colors);

    print_bottom_border(inner_width, &colors);
}

/* =======================================================================
                         --classification-help option
======================================================================= */
pub fn print_classification_help(colors_enabled: bool) {
    let inner_width = manual_inner_width();

    let colors = ManualColors::new(colors_enabled);

    print_top_border(inner_width, &colors);

    print_line(
        "Nightkrawler Classification Help",
        inner_width,
        colors.title,
        &colors,
    );

    print_separator(inner_width, &colors);

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Nightkrawler classifies duplicate paths as",
                color: colors.text,
            },
            ManualPart {
                text: "ordinary, generated,",
                color: colors.accent,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "protected.",
                color: colors.accent,
            },
            ManualPart {
                text: "Classification helps make scan results easier to evaluate and adds safety around filesystem areas that deserve additional protection.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_section("Classes", inner_width, &colors);

    print_manual_subsection(
        "Ordinary",
        &[ManualPart {
            text: "An ordinary path does not belong to a generated or protected tree. Most normal user files will fall into this class.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_subsection(
        "Generated",
        &[ManualPart {
            text: "A generated path contains a directory name commonly associated with generated, cached, or rebuildable content.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "Recognized generated directory names include",
                color: colors.text,
            },
            ManualPart {
                text: "target, build, dist, .cache, __pycache__, node_modules,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: ".gradle.",
                color: colors.accent,
            },
        ],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_indented_paragraph(
        &[ManualPart {
            text: "Generated classification does not automatically remove or ignore a file. It is a classification used for reporting and evaluation.",
            color: colors.text,
        }],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_subsection(
        "Protected",
        &[ManualPart {
            text: "A protected path belongs to a filesystem tree where Nightkrawler action modes require explicit permission before operating on removable duplicate candidates.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "Built-in protected roots include",
                color: colors.text,
            },
            ManualPart {
                text: "/boot, /etc,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "/root.",
                color: colors.accent,
            },
        ],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    #[cfg(target_os = "freebsd")]
    print_indented_paragraph(
        &[
            ManualPart {
                text: "On FreeBSD,",
                color: colors.text,
            },
            ManualPart {
                text: "/usr/local/etc",
                color: colors.accent,
            },
            ManualPart {
                text: "is protected by default.",
                color: colors.text,
            },
        ],
        inner_width,
        4,
        &colors,
    );

    #[cfg(target_os = "freebsd")]
    print_blank_line(inner_width, &colors);

    #[cfg(target_os = "netbsd")]
    print_indented_paragraph(
        &[
            ManualPart {
                text: "On NetBSD,",
                color: colors.text,
            },
            ManualPart {
                text: "/usr/pkg/etc",
                color: colors.accent,
            },
            ManualPart {
                text: "is protected by default.",
                color: colors.text,
            },
        ],
        inner_width,
        4,
        &colors,
    );

    #[cfg(target_os = "netbsd")]
    print_blank_line(inner_width, &colors);

    print_indented_paragraph(
        &[
            ManualPart {
                text: "When",
                color: colors.text,
            },
            ManualPart {
                text: "HOME",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is available, Nightkrawler also protects",
                color: colors.text,
            },
            ManualPart {
                text: "~/.ssh, ~/.gnupg,",
                color: colors.accent,
            },
            ManualPart {
                text: "and",
                color: colors.text,
            },
            ManualPart {
                text: "~/.config.",
                color: colors.accent,
            },
        ],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_section("Group Classification", inner_width, &colors);

    print_manual_paragraph(
        &[ManualPart {
            text: "Classification applies to complete duplicate groups as well as individual paths. If a group contains members from different classes, Nightkrawler reports the group using the most restrictive class present.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[
            ManualPart {
                text: "Protected",
                color: colors.accent,
            },
            ManualPart {
                text: "takes precedence over",
                color: colors.text,
            },
            ManualPart {
                text: "Generated,",
                color: colors.accent,
            },
            ManualPart {
                text: "which takes precedence over",
                color: colors.text,
            },
            ManualPart {
                text: "Ordinary.",
                color: colors.accent,
            },
        ],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_section("Custom Protection", inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Use",
                color: colors.text,
            },
            ManualPart {
                text: "--protect",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "-P",
                color: colors.emphasis,
            },
            ManualPart {
                text: "to add one or more protected directory trees for the current scan.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_indented_paragraph(
        &[ManualPart {
            text: "nk ~ -R --min-size 100M --protect ~/.sensitive ~/important",
            color: colors.example,
        }],
        inner_width,
        4,
        &colors,
    );

    print_blank_line(inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Protection is not the same as exclusion. A path supplied with",
                color: colors.text,
            },
            ManualPart {
                text: "--exclude",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is not scanned. A path supplied with",
                color: colors.text,
            },
            ManualPart {
                text: "--protect",
                color: colors.emphasis,
            },
            ManualPart {
                text: "may still be scanned, classified, and reported.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_section("Protection and Actions", inner_width, &colors);

    print_manual_paragraph(
        &[
            ManualPart {
                text: "Report-only scans do not require",
                color: colors.text,
            },
            ManualPart {
                text: "--allow-protected.",
                color: colors.emphasis,
            },
            ManualPart {
                text: "Protected duplicate groups may still be inspected and reported normally.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "If",
                color: colors.text,
            },
            ManualPart {
                text: "--quarantine, --trash,",
                color: colors.emphasis,
            },
            ManualPart {
                text: "or",
                color: colors.text,
            },
            ManualPart {
                text: "--delete",
                color: colors.emphasis,
            },
            ManualPart {
                text: "would act on a protected removable candidate, Nightkrawler performs a safety stop unless",
                color: colors.text,
            },
            ManualPart {
                text: "--allow-protected",
                color: colors.emphasis,
            },
            ManualPart {
                text: "was supplied.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[ManualPart {
            text: "The safety stop reports the protected candidate, its protected root, and the scan path. The refused action does not change original files.",
            color: colors.text,
        }],
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "--allow-protected",
                color: colors.emphasis,
            },
            ManualPart {
                text: "is an explicit override. It tells Nightkrawler that action on protected removable candidates is intentional.",
                color: colors.text,
            },
        ],
        inner_width,
        &colors,
    );

    print_manual_section("Examples", inner_width, &colors);

    print_example(
        &[
            ManualPart {
                text: "Scan normally while treating",
                color: colors.text,
            },
            ManualPart {
                text: "~/.sensitive",
                color: colors.accent,
            },
            ManualPart {
                text: "as protected:",
                color: colors.text,
            },
        ],
        "nk ~ -R --min-size 100M --protect ~/.sensitive",
        inner_width,
        &colors,
    );

    print_example(
        &[
            ManualPart {
                text: "Scan",
                color: colors.text,
            },
            ManualPart {
                text: "/",
                color: colors.accent,
            },
            ManualPart {
                text: "and explicitly permit quarantine action on protected candidates:",
                color: colors.text,
            },
        ],
        "nk / -R --min-size 100M -q /tmp/q --allow-protected",
        inner_width,
        &colors,
    );

    print_example(
        &[ManualPart {
            text: "Exclude a tree entirely instead of protecting it:",
            color: colors.text,
        }],
        "nk ~ -R --min-size 100M --exclude ~/.cache",
        inner_width,
        &colors,
    );

    print_manual_paragraph(
        &[
            ManualPart {
                text: "For the complete Nightkrawler documentation, use",
                color: colors.text,
            },
            ManualPart {
                text: "--manual.",
                color: colors.emphasis,
            },
        ],
        inner_width,
        &colors,
    );

    print_bottom_border(inner_width, &colors);
}

pub fn terminal_width() -> usize {
    terminal_width_from_ioctl()
        .or_else(terminal_width_from_environment)
        .unwrap_or(80)
}

fn manual_inner_width() -> usize {
    let available = terminal_width().saturating_sub(8);

    available.clamp(MIN_INNER_WIDTH, MAX_INNER_WIDTH)
}

fn terminal_width_from_environment() -> Option<usize> {
    env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|width| *width > 0)
}

fn terminal_width_from_ioctl() -> Option<usize> {
    for file_descriptor in [libc::STDOUT_FILENO, libc::STDERR_FILENO, libc::STDIN_FILENO] {
        let mut window_size = libc::winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let result = unsafe { libc::ioctl(file_descriptor, libc::TIOCGWINSZ, &mut window_size) };

        if result == 0 {
            let width = window_size.ws_col as usize;

            if width > 0 {
                return Some(width);
            }
        }
    }

    None
}

fn print_top_border(width: usize, colors: &ManualColors) {
    if use_ascii_frame() {
        println!(
            "{}+{}+{}",
            colors.border,
            "-".repeat(width + 4),
            colors.reset,
        );
    } else {
        println!(
            "{}┌{}┐{}",
            colors.border,
            "─".repeat(width + 4),
            colors.reset,
        );
    }
}

fn print_separator(width: usize, colors: &ManualColors) {
    if use_ascii_frame() {
        println!(
            "{}+{}+{}",
            colors.border,
            "-".repeat(width + 4),
            colors.reset,
        );
    } else {
        println!(
            "{}├{}┤{}",
            colors.border,
            "─".repeat(width + 4),
            colors.reset,
        );
    }
}

fn print_bottom_border(width: usize, colors: &ManualColors) {
    if use_ascii_frame() {
        println!(
            "{}+{}+{}",
            colors.border,
            "-".repeat(width + 4),
            colors.reset,
        );
    } else {
        println!(
            "{}└{}┘{}",
            colors.border,
            "─".repeat(width + 4),
            colors.reset,
        );
    }
}

fn print_blank_line(width: usize, colors: &ManualColors) {
    if use_ascii_frame() {
        println!(
            "{}+{}+{}",
            colors.border,
            "-".repeat(width + 4),
            colors.reset,
        );
    } else {
        println!(
            "{}│{}  {}  {}│{}",
            colors.border,
            colors.reset,
            " ".repeat(width),
            colors.border,
            colors.reset,
        );
    }
}

fn print_line(text: &str, width: usize, color: &str, colors: &ManualColors) {
    let visible_width = text.chars().count();

    let padding = width.saturating_sub(visible_width);

    if use_ascii_frame() {
        println!(
            "{}|{}  {}{}{}{}  {}|{}",
            colors.border,
            colors.reset,
            color,
            text,
            colors.reset,
            " ".repeat(padding),
            colors.border,
            colors.reset,
        );
    } else {
        println!(
            "{}│{}  {}{}{}{}  {}│{}",
            colors.border,
            colors.reset,
            color,
            text,
            colors.reset,
            " ".repeat(padding),
            colors.border,
            colors.reset,
        );
    }
}

fn print_option_line(option: &str, description: &str, width: usize, colors: &ManualColors) {
    const INDENT: usize = 2;
    const OPTION_WIDTH: usize = 28;

    let content_width = width.saturating_sub(INDENT);

    print_left_border_with_indent(INDENT, colors);

    let option_width = option.chars().count();

    print!("{}{}{}", colors.option, option, colors.reset,);

    let gap = OPTION_WIDTH.saturating_sub(option_width);

    print!("{}", " ".repeat(gap));

    print!("{}{}{}", colors.text, description, colors.reset,);

    let used_width = OPTION_WIDTH + description.chars().count();

    print_right_padding(content_width, used_width, colors);
}

fn print_indented_paragraph(
    parts: &[ManualPart<'_>],
    width: usize,
    indent: usize,
    colors: &ManualColors,
) {
    let content_width = width.saturating_sub(indent);

    let mut current_width = 0usize;

    print_left_border_with_indent(indent, colors);

    for part in parts {
        for word in part.text.split_whitespace() {
            let word_width = word.chars().count();

            let separator_width = usize::from(current_width > 0);

            if current_width + separator_width + word_width > content_width {
                print_right_padding(content_width, current_width, colors);

                current_width = 0;

                print_left_border_with_indent(indent, colors);
            }

            if current_width > 0 {
                print!(" ");

                current_width += 1;
            }

            if part.color.is_empty() {
                print!("{}{}{}", colors.text, word, colors.reset,);
            } else {
                print!("{}{}{}", part.color, word, colors.reset);
            }

            current_width += word_width;
        }
    }

    print_right_padding(content_width, current_width, colors);
}

fn print_example(
    description: &[ManualPart<'_>],
    command: &str,
    inner_width: usize,
    colors: &ManualColors,
) {
    print_indented_paragraph(description, inner_width, 2, colors);

    print_indented_paragraph(
        &[ManualPart {
            text: command,
            color: colors.example,
        }],
        inner_width,
        4,
        colors,
    );

    print_blank_line(inner_width, colors);
}

fn print_manual_section(title: &str, inner_width: usize, colors: &ManualColors) {
    print_separator(inner_width, colors);

    print_blank_line(inner_width, colors);

    print_line(title, inner_width, colors.title, colors);

    print_blank_line(inner_width, colors);
}

fn print_manual_paragraph(parts: &[ManualPart<'_>], inner_width: usize, colors: &ManualColors) {
    print_indented_paragraph(parts, inner_width, 2, colors);

    print_blank_line(inner_width, colors);
}

fn print_manual_subsection(
    title: &str,
    parts: &[ManualPart<'_>],
    inner_width: usize,
    colors: &ManualColors,
) {
    print_indented_paragraph(
        &[ManualPart {
            text: title,
            color: colors.accent,
        }],
        inner_width,
        2,
        colors,
    );

    print_indented_paragraph(parts, inner_width, 4, colors);

    print_blank_line(inner_width, colors);
}

fn print_left_border_with_indent(indent: usize, colors: &ManualColors) {
    if use_ascii_frame() {
        print!(
            "{}|{}  {}",
            colors.border,
            colors.reset,
            " ".repeat(indent),
        );
    } else {
        print!(
            "{}│{}  {}",
            colors.border,
            colors.reset,
            " ".repeat(indent),
        );
    }
}

fn print_right_padding(width: usize, current_width: usize, colors: &ManualColors) {
    let padding = width.saturating_sub(current_width);

    if use_ascii_frame() {
        println!(
            "{}{}|{}",
            " ".repeat(padding + 2),
            colors.border,
            colors.reset,
        );
    } else {
        println!(
            "{}{}│{}",
            " ".repeat(padding + 2),
            colors.border,
            colors.reset,
        );
    }
}

fn use_ascii_frame() -> bool {
    if env::consts::OS != "netbsd" || !std::io::stdout().is_terminal() {
        return false;
    }

    let tty_name = unsafe { libc::ttyname(libc::STDOUT_FILENO) };

    if tty_name.is_null() {
        return false;
    }

    let tty_path = unsafe { CStr::from_ptr(tty_name) }
        .to_string_lossy();

    tty_path.starts_with("/dev/ttyE")
        || tty_path == "/dev/console"
        || tty_path == "/dev/constty"
}