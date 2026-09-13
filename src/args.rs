// SPDX-License-Identifier: BSD-3-Clause

use crate::model::{Command, KeepPolicy, Options};
use std::env;
use std::path::PathBuf;
use std::io::IsTerminal;

use crate::manual::{
    print_classification_help,
    print_help,
    print_manual,
};
pub fn parse_arguments(arguments: &[String]) -> Command {
    if arguments.is_empty() {
        print_usage_and_exit();
    }

    match arguments[0].as_str() {
        "--restore" | "-u" => {
            return parse_restore_command(arguments);
        }

        "--cleanup" | "-c" => {
            return parse_cleanup_command(arguments);
        }

        _ => {}
    }

    Command::Scan(parse_scan_arguments(arguments))
}

fn parse_scan_arguments(arguments: &[String]) -> Options {
    let mut root_path: Option<PathBuf> = None;

    let mut recursive = false;

    let mut min_size: Option<u64> = None;

    let mut force_dotdirs = false;

    let mut allow_protected = false;

    let mut trash = false;

    let mut delete = false;

    let mut excludes = Vec::new();

    let mut protected_paths = Vec::new();

    let mut prefer_paths = Vec::new();

    let mut keep_policy = KeepPolicy::Default;

    let mut quarantine_path: Option<PathBuf> = None;

    let mut report_path: Option<PathBuf> = None;

    let mut interactive = false;

    let mut use_colors =
        std::io::stdout().is_terminal()
            && std::env::var_os("NO_COLOR").is_none();

    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "-h" | "--help" => {
                print_help(use_colors);
                std::process::exit(0);
            }

            "-v" | "--version" => {
                println!(
                    "nightkrawler {}",
                    env!("CARGO_PKG_VERSION"),
                );

                std::process::exit(0);
            }

            "--manual" => {
                print_manual(use_colors);

                std::process::exit(0);
            }

            "--classification-help" => {
                print_classification_help(use_colors);

                std::process::exit(0);
            }

            "--recursive" | "-R" => {
                recursive = true;
                index += 1;
            }

            "--force-dotdirs" => {
                force_dotdirs = true;
                index += 1;
            }

            "--allow-protected" => {
                allow_protected = true;

                index += 1;
            }

            "--keep-highest-version" => {
                keep_policy = KeepPolicy::HighestVersion;

                index += 1;
            }

            "--interactive" | "-i" => {
                interactive = true;

                index += 1;
            }

            "--no-color" | "--no-colour" | "--no-colors" | "--no-colours" => {
                use_colors = false;

                index += 1;
            }

            "--prefer" | "-p" => {
                index += 1;

                if index >= arguments.len() {
                    eprintln!("nightkrawler: --prefer requires at least one path");

                    std::process::exit(2);
                }

                while index < arguments.len() && !arguments[index].starts_with('-') {
                    prefer_paths.push(expand_home_path(&arguments[index]));

                    index += 1;
                }
            }

            "--min-size" => {
                if index + 1 >= arguments.len() {
                    eprintln!("nightkrawler: --min-size requires a size, for example 10M");

                    std::process::exit(2);
                }

                let Some(size) = parse_size_argument(&arguments[index + 1]) else {
                    eprintln!("nightkrawler: invalid size '{}'", arguments[index + 1]);

                    std::process::exit(2);
                };

                min_size = Some(size);

                index += 2;
            }

            "--exclude" | "-E" => {
                index += 1;

                if index >= arguments.len() {
                    eprintln!("nightkrawler: --exclude requires at least one path");

                    std::process::exit(2);
                }

                while index < arguments.len() && !arguments[index].starts_with('-') {
                    excludes.push(expand_home_path(&arguments[index]));

                    index += 1;
                }
            }

            "--protect" | "-P" => {
                index += 1;

                if index >= arguments.len() {
                    eprintln!("nightkrawler: --protect requires at least one path");

                    std::process::exit(2);
                }

                while index < arguments.len() && !arguments[index].starts_with('-') {
                    protected_paths.push(expand_home_path(&arguments[index]));

                    index += 1;
                }
            }

            "--trash" | "-t" => {
                trash = true;

                index += 1;
            }

            "--delete" | "-d" => {
                delete = true;

                index += 1;
            }

            "--quarantine" | "-q" => {
                if index + 1 >= arguments.len() {
                    eprintln!("nightkrawler: --quarantine requires a path");

                    std::process::exit(2);
                }

                quarantine_path = Some(expand_home_path(&arguments[index + 1]));

                index += 2;
            }

            "--report" | "-r" => {
                if index + 1 >= arguments.len() {
                    eprintln!("nightkrawler: --report requires a directory path");

                    std::process::exit(2);
                }

                report_path = Some(expand_home_path(&arguments[index + 1]));

                index += 2;
            }

            argument if argument.starts_with('-') => {
                eprintln!("nightkrawler: unknown option '{}'", argument);

                std::process::exit(2);
            }

            argument => {
                if root_path.is_some() {
                    eprintln!("nightkrawler: only one scan path is allowed");

                    std::process::exit(2);
                }

                root_path = Some(expand_home_path(argument));

                index += 1;
            }
        }
    }

    let Some(root_path) = root_path else {
        eprintln!("nightkrawler: missing scan path");

        std::process::exit(2);
    };

    let Some(min_size) = min_size else {
        eprintln!("nightkrawler: --min-size is required");

        eprintln!("example: nk ~/Downloads --recursive --min-size 100M");

        std::process::exit(2);
    };

    let options = Options {
        root_path,
        recursive,
        min_size,
        force_dotdirs,
        allow_protected,
        delete,
        trash,
        excludes,
        protected_paths,
        prefer_paths,
        keep_policy,
        quarantine_path,
        report_path,
        interactive,
        use_colors,
    };

    if let Err(message) = crate::validation::validate_scan_options(&options) {
        eprintln!("nightkrawler: {}", message);

        std::process::exit(2);
    }

    options
}

fn parse_restore_command(arguments: &[String]) -> Command {
    if arguments.len() < 2 {
        eprintln!("nightkrawler: --restore requires a quarantine path");

        std::process::exit(2);
    }

    if arguments.len() > 2 {
        eprintln!("nightkrawler: --restore accepts only one quarantine path");

        std::process::exit(2);
    }

    Command::Restore(expand_home_path(&arguments[1]))
}

fn parse_cleanup_command(arguments: &[String]) -> Command {
    if arguments.len() < 2 {
        eprintln!("nightkrawler: --cleanup requires a quarantine path");

        std::process::exit(2);
    }

    if arguments.len() > 2 {
        eprintln!("nightkrawler: --cleanup accepts only one quarantine path");

        std::process::exit(2);
    }

    Command::Cleanup(expand_home_path(&arguments[1]))
}

fn print_usage_and_exit() {
    let use_colors =
        std::io::stdout().is_terminal()
            && std::env::var_os("NO_COLOR").is_none();

    print_help(use_colors);

    std::process::exit(2);
}

fn expand_home_path(value: &str) -> PathBuf {
    if value == "~" {
        if let Some(home) = env::var_os("HOME") {
            return PathBuf::from(home);
        }
    }

    if let Some(rest) = value.strip_prefix("~/") {
        if let Some(home) = env::var_os("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }

    PathBuf::from(value)
}

fn parse_size_argument(value: &str) -> Option<u64> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return None;
    }

    let mut number_part = String::new();

    let mut unit_part = String::new();

    for character in trimmed.chars() {
        if character.is_ascii_digit() {
            if !unit_part.is_empty() {
                return None;
            }

            number_part.push(character);
        } else {
            unit_part.push(character);
        }
    }

    if number_part.is_empty() {
        return None;
    }

    let number = number_part.parse::<u64>().ok()?;

    let multiplier = match unit_part.to_lowercase().as_str() {
        "" | "b" => 1,
        "k" | "kb" | "kib" => 1024,
        "m" | "mb" | "mib" => 1024 * 1024,
        "g" | "gb" | "gib" => 1024 * 1024 * 1024,
        "t" | "tb" | "tib" => 1024_u64 * 1024 * 1024 * 1024,

        _ => {
            return None;
        }
    };

    number.checked_mul(multiplier)
}
