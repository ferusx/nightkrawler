// SPDX-License-Identifier: BSD-3-Clause

use crate::model::Options;

pub fn validate_scan_options(options: &Options) -> Result<(), String> {
    if options.delete && options.trash {
        return Err("choose either --trash or --delete, not both".to_string());
    }

    if options.interactive && !options.delete && !options.trash {
        return Err("--interactive requires --trash or --delete".to_string());
    }

    if options.allow_protected
        && !options.delete
        && !options.trash
        && options.quarantine_path.is_none()
    {
        return Err("--allow-protected requires --quarantine, --trash, or --delete".to_string());
    }

    Ok(())
}
