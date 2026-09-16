# Nightkrawler

Nightkrawler is a filesystem duplicate finder and removal tool for UNIX-like systems.

It scans regular files for identical content, selects one file from each duplicate group to keep, and identifies the remaining copies as removable duplicates. By default, Nightkrawler is report-only and does not modify the original files.

Nightkrawler is designed around cautious inspection, explicit action modes, recoverable workflows, path classification, and additional safeguards around sensitive filesystem areas.

## Features

- Finds duplicate files by content rather than filename.
- Groups files by size before hashing to avoid unnecessary work.
- Verifies hash matches with direct byte-for-byte comparison.
- Operates in report-only mode by default.
- Supports recursive and non-recursive scanning.
- Requires an explicit minimum file-size threshold for scans.
- Can exclude selected paths from scanning.
- Skips symbolic links.
- Skips dot-directories during recursive scans unless explicitly enabled.
- Uses deterministic keeper selection.
- Supports ordered preferred keeper paths.
- Can prefer the highest version-like pathname component.
- Classifies duplicate paths as Ordinary, Generated, or Protected.
- Includes built-in protection for sensitive filesystem trees.
- Supports additional user-defined protected paths.
- Can quarantine removable duplicates when explicitly requested with `--quarantine`.
- Can combine quarantine with Trash or permanent deletion.
- Refuses the destructive stage if a preceding quarantine copy is incomplete.
- Checks available free space before quarantine copying.
- Can restore files from a Nightkrawler quarantine.
- Provides guarded quarantine cleanup.
- Supports the system Trash implementation.
- Supports permanent deletion.
- Supports interactive confirmation for Trash and delete operations.
- Produces detailed scan reports.
- Supports colored terminal output and no-color operation.
- Includes built-in compact help, a full manual, and classification-specific help.
- Packaged installations provide both `nightkrawler` and the short `nk` command.

## Safety Model

Nightkrawler separates inspection from action.

Without `--quarantine`, `--trash`, or `--delete`, Nightkrawler runs in report-only mode. It scans, classifies, and reports duplicate groups without changing the original files.

One verified duplicate from every group is always selected as the keeper. Action modes operate only on the remaining removable candidates.

Protected paths add another safety layer. If an action mode would operate on a protected removable candidate, Nightkrawler stops unless `--allow-protected` was explicitly supplied.

Quarantine may be combined with Trash or permanent deletion. In that workflow, Nightkrawler first copies all intended removable candidates into quarantine. If the quarantine stage fails or is incomplete, the Trash or delete stage is refused.

Nightkrawler does not follow symbolic links.

## Supported Platforms

Nightkrawler is intended for UNIX-like systems.

The current code contains platform-specific protected-path handling for:

- Linux and other UNIX-like systems
- FreeBSD
- NetBSD

The program uses UNIX APIs and filesystem semantics and is not intended as a native Windows application.

## Requirements

### Packaged installation

Users who install a finished package do **not** need Rust or Cargo installed. The package contains the already-built Nightkrawler executable.

### Building from source

Building Nightkrawler from source requires:

- Rust
- Cargo
- a standard UNIX-like build environment

Nightkrawler currently uses Rust edition 2024.

The crate metadata declares these direct Rust dependencies:

- `libc = 0.2.186`
- `trash = 5.2.6`

A `Cargo.lock` file is included in the release source so builds can use the locked dependency set.

## Installation

### From a distribution package

When a package is available for your distribution, use the native package manager.

For a locally built Arch Linux package:

```sh
sudo pacman -U nightkrawler-*.pkg.tar.zst
```

For a locally built RPM package on openSUSE:

```sh
sudo zypper install ./nightkrawler-*.rpm
```

Packaged installations install the main executable and the short command:

```text
nightkrawler
nk
```

They also install the manual page:

```sh
man nightkrawler
```

### Build from source

Clone the repository:

```sh
git clone https://github.com/ferusx/nightkrawler.git
cd nightkrawler
```

Build the release binary using the locked dependency set:

```sh
cargo build --release --locked
```

The resulting executable is:

```text
target/release/nightkrawler
```

### Optional manual installation from source

If you deliberately want to install a locally built copy without creating a distribution package:

```sh
sudo install -Dm0755 target/release/nightkrawler /usr/local/bin/nightkrawler
sudo ln -s nightkrawler /usr/local/bin/nk
sudo install -Dm0644 man/man1/nightkrawler.1 /usr/local/share/man/man1/nightkrawler.1
```

This manual method is mainly useful for local development or testing. Distribution packages are preferable for normal system installation because the package manager can track, upgrade, and remove installed files cleanly.

## Usage

```text
nk PATH --min-size SIZE [options]
nightkrawler PATH --min-size SIZE [options]
```

`PATH` is the directory or file tree to inspect.

Only one scan path may be supplied.

`--min-size` is required for scan commands.

Accepted size suffixes include:

```text
B
K
KB
KiB
M
MB
MiB
G
GB
GiB
T
TB
TiB
```

Examples:

```text
100K
10M
1G
1T
```

## Command Reference

### Meta options

| Option | Description |
|---|---|
| `-h`, `--help` | Show compact help. |
| `-v`, `--version` | Show the Nightkrawler version. |
| `--manual` | Show the full built-in manual. |
| `--classification-help` | Show focused help about path classification. |
| `--no-color`, `--no-colour`, `--no-colors`, `--no-colours` | Disable colored output. |

### Scan options

| Option | Description |
|---|---|
| `--min-size SIZE` | Required. Ignore files smaller than `SIZE`. |
| `-R`, `--recursive` | Scan recursively. |
| `--force-dotdirs` | Include dot-directories during recursive scans. |
| `-E`, `--exclude PATH...` | Exclude one or more paths from scanning. |
| `-P`, `--protect PATH...` | Add one or more protected path trees. |
| `--allow-protected` | Permit action modes to operate on protected removable candidates. |
| `-p`, `--prefer PATH...` | Give one or more path trees keeper priority. |
| `--keep-highest-version` | Prefer the highest version-like pathname component when selecting a keeper. |
| `-q`, `--quarantine PATH` | Copy removable duplicates into a quarantine directory. |
| `-t`, `--trash` | Move removable duplicates to the system Trash. |
| `-d`, `--delete` | Permanently delete removable duplicates. |
| `-i`, `--interactive` | Ask before each Trash or delete action. |
| `-r`, `--report PATH` | Write the report to a selected path or directory. |

### Quarantine management

| Option | Description |
|---|---|
| `-u`, `--restore PATH` | Restore files recorded in a Nightkrawler quarantine. |
| `-c`, `--cleanup PATH` | Remove a recognized Nightkrawler quarantine directory after it is no longer needed. |

## Scanning

Nightkrawler scans regular files only.

Without `--recursive` or `-R`, it examines files directly under the selected scan path.

With recursive scanning enabled, Nightkrawler descends into subdirectories.

Dot-directories are skipped during recursive scanning unless `--force-dotdirs` is supplied.

Paths supplied with `--exclude` or `-E` are omitted from scanning.

When a quarantine directory is configured, Nightkrawler also excludes that quarantine tree from the scan so copied quarantine files cannot immediately become duplicate candidates.

Symbolic links are not followed and are not added to duplicate groups.

## Duplicate Detection

Nightkrawler determines duplicates in several stages:

1. Files are grouped by size.
2. Files with unique sizes require no further duplicate comparison.
3. Matching-size files are hashed.
4. Files sharing the same hash are verified with direct byte-for-byte comparison.
5. Only files confirmed as identical are placed in the same duplicate group.

The final byte comparison means a hash match alone is never treated as proof that two files are identical.

Filenames do not need to match. Files with completely different names can belong to the same duplicate group when their contents are identical.

Zero-byte files follow the same duplicate rule as every other regular file.

## Keeper Selection

Every duplicate group contains one keeper. The remaining verified copies become removable candidates.

Keeper selection does not imply that the other files are different. Every member of the group has already been verified as identical by content.

### Default policy

The default keeper policy uses deterministic path-based ordering.

Files under locations commonly associated with temporary or replaceable content are less preferred than ordinary locations.

Examples include:

- `Downloads`
- `.cache`
- `.cargo`
- `.gradle`
- `.local/share`

When candidates remain tied, Nightkrawler uses path depth and pathname ordering to make the final choice reproducible.

### Preferred paths

Use `--prefer` or `-p` to give selected directory trees explicit keeper priority:

```sh
nk ~ -R --min-size 100M --prefer ~/DevX ~/Documents
```

Preferred paths are ordered. The first supplied path has the highest priority, the second has the next priority, and so on.

### Highest-version policy

Use:

```text
--keep-highest-version
```

to prefer the file associated with the highest version-like pathname component.

Recognized examples include:

```text
1.2
2.0.1
v3.4
```

Explicit `--prefer` paths take priority before the selected keeper policy is applied.

## Classification

Nightkrawler classifies duplicate paths as:

- Ordinary
- Generated
- Protected

Classification helps separate normal files from generated trees and sensitive filesystem areas.

### Ordinary

An Ordinary path does not belong to a Generated or Protected tree.

Most normal user files fall into this class.

### Generated

Generated paths are recognized when their pathname contains a directory name commonly associated with generated, cached, or rebuildable content.

Recognized generated directory names include:

- `target`
- `build`
- `dist`
- `.cache`
- `__pycache__`
- `node_modules`
- `.gradle`

Generated classification does **not** automatically remove or ignore a file. It is used for reporting and evaluation.

### Protected

Protected paths belong to filesystem trees where action modes require explicit permission.

Built-in protected roots include:

- `/boot`
- `/etc`
- `/root`

When `HOME` is available, Nightkrawler also protects:

- `~/.ssh`
- `~/.gnupg`
- `~/.config`

On FreeBSD, `/usr/local/etc` is also protected by default.

On NetBSD, `/usr/pkg/etc` is also protected by default.

Additional protected trees may be supplied with:

```text
--protect PATH...
```

If a duplicate group contains files from multiple classifications, Nightkrawler reports the complete group using the most restrictive class present:

```text
Protected > Generated > Ordinary
```

For focused classification documentation:

```sh
nk --classification-help
```

## Protection

Protection is different from exclusion.

An excluded path is not scanned.

A protected path may still be scanned, classified, and reported. Protection becomes decisive only when an action mode would operate on a protected removable candidate.

Example:

```sh
nk ~ -R --min-size 100M --protect ~/.sensitive
```

If an action mode encounters a protected removable candidate, Nightkrawler stops unless:

```text
--allow-protected
```

was supplied.

The safety stop reports information including the protected candidate, the matching protected root, and the scan path.

The refused action does not change the original files.

`--allow-protected` is an explicit override and should only be used when action on protected removable candidates is intentional.

## Action Modes

Nightkrawler has three action modes:

- Quarantine
- Trash
- Delete

Without an action mode, a scan is report-only.

### Quarantine

```text
-q PATH
--quarantine PATH
```

Quarantine copies removable duplicates into the selected quarantine directory while leaving the original files in place.

Nightkrawler preserves the scanned path structure below the quarantine directory where possible.

If a destination filename already exists, Nightkrawler creates a unique Nightkrawler-suffixed destination instead of overwriting the existing quarantine file.

Before copying begins, Nightkrawler calculates the total size of the removable duplicates and checks available free space on the filesystem containing the quarantine directory.

If there is not enough space, quarantine is refused before the copy stage proceeds.

A quarantine manifest records:

- original path
- quarantine path
- file size

The manifest stores UNIX pathnames as raw pathname bytes rather than using tab- or newline-separated text fields. This allows valid filenames containing spaces, tabs, newlines, or non-UTF-8 bytes to be represented safely.

### Trash

```text
-t
--trash
```

Trash moves removable duplicates to the system Trash implementation.

Before the operation begins, Nightkrawler performs a Trash preflight check for the relevant filesystem.

If Trash is unavailable, the operation is stopped.

### Delete

```text
-d
--delete
```

Delete permanently removes removable duplicate files.

This mode should be used carefully.

### Combining quarantine with Trash or delete

Quarantine may be combined with either Trash or permanent deletion.

Example:

```sh
nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine -t
```

or:

```sh
nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine -d
```

In either workflow, Nightkrawler copies the removable duplicates into quarantine first.

The Trash or delete stage is allowed to proceed only if every intended quarantine copy completed successfully.

If quarantine copying fails or is incomplete, Nightkrawler performs a safety stop and refuses the destructive stage.

### Mutually exclusive actions

`--trash` and `--delete` cannot be used together.

Choose one destructive action for a scan.

## Interactive Mode

Use:

```text
-i
--interactive
```

with Trash or delete to ask before each destructive action.

Examples:

```sh
nk ~/Downloads -R --min-size 100M -t -i
```

```sh
nk ~/Downloads -R --min-size 100M -d -i
```

Interactive mode does not make quarantine copying interactive.

## Restore

Restore files from a Nightkrawler quarantine with:

```sh
nk --restore ~/nk-quarantine
```

or:

```sh
nk -u ~/nk-quarantine
```

Nightkrawler reads the quarantine manifest and restores files to their original paths.

If an original destination already exists, that record is skipped rather than overwritten.

If a required parent directory no longer exists, Nightkrawler recreates it before restoring the file.

Restore copies files back to their original locations. The quarantine remains available until it is cleaned separately.

## Cleanup

Remove a Nightkrawler quarantine after it is no longer needed:

```sh
nk --cleanup ~/nk-quarantine
```

or:

```sh
nk -c ~/nk-quarantine
```

Cleanup includes additional safeguards because removing an entire directory tree carries different risks from ordinary duplicate processing.

Nightkrawler refuses to clean:

- the filesystem root `/`
- the user's home directory
- the current working directory

A missing or empty quarantine is treated as having nothing to clean.

Before removing a populated directory, cleanup verifies that it contains a recognized Nightkrawler quarantine manifest.

## Reports

When duplicate groups are found, Nightkrawler writes a detailed text report.

Without `--report` or `-r`, the report is created in the current directory with an automatically generated filename.

To select a different destination:

```sh
nk ~/Downloads -R --min-size 100M --report ~/reports
```

If the supplied path is an existing directory, Nightkrawler places an automatically named report inside it.

Otherwise, the supplied path is used as the report filename.

Reports include information such as:

- scan date
- operating system
- architecture
- hostname
- total scan duration
- scan root
- minimum size
- recursive state
- action mode
- keeper policy
- excluded paths
- protected paths
- duplicate groups
- keeper paths
- removable candidates
- reclaimable space
- quarantine details when applicable

Duplicate groups are divided into Protected, Generated, and Ordinary sections when those classifications are present.

## Examples

**Report duplicates in a directory:**

```sh
nk ~/Downloads --min-size 100M
```

**Scan recursively:**

```sh
nk ~/Downloads -R --min-size 100M
```

**Scan the home directory recursively:**

```sh
nk ~ -R --min-size 100M
```

**Exclude selected directory trees:**

```sh
nk ~ -R --min-size 100M -E ~/bin ~/PycharmProjects ~/snap
```

**Include dot-directories:**

```sh
nk ~ -R --force-dotdirs --min-size 100M
```

**Prefer keeping files under selected paths:**

```sh
nk ~ -R --min-size 100M -p ~/DevX ~/Documents
```

**Prefer the highest version-like pathname:**

```sh
nk ~/Downloads -R --min-size 100M --keep-highest-version
```

**Protect an additional tree:**

```sh
nk ~ -R --min-size 100M --protect ~/.sensitive
```

**Quarantine removable duplicates:**

```sh
nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine
```

**Move removable duplicates to Trash:**

```sh
nk ~/Downloads -R --min-size 100M --trash
```

**Offer each removable duplicate for Trash interactively:**

```sh
nk ~/Downloads -R --min-size 100M -t -i
```

**Quarantine first, then move originals to Trash:**

```sh
nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine -t
```

**Permanently delete removable duplicates:**

```sh
nk ~/Downloads -R --min-size 100M --delete
```

**Offer each removable duplicate for deletion interactively:**

```sh
nk ~/Downloads -R --min-size 100M -d -i
```

**Quarantine first, then permanently delete originals:**

```sh
nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine -d
```

**Quarantine first, then offer each original for deletion:**

```sh
nk ~/Downloads -R --min-size 100M -q ~/nk-quarantine -d -i
```

**Permit action on protected candidates deliberately:**

```sh
nk / -R --min-size 100M -q /tmp/q --allow-protected
```

**Restore a quarantine:**

```sh
nk --restore ~/nk-quarantine
```

**Clean a quarantine:**

```sh
nk --cleanup ~/nk-quarantine
```

**Write a report to a selected location:**

```sh
nk ~/Downloads -R --min-size 100M -r ~/reports
```

**Disable colors:**

```sh
nk ~/Downloads -R --min-size 100M --no-color
```

## Built-in Documentation

Compact help:

```sh
nk --help
```

Full built-in manual:

```sh
nk --manual
```

Classification reference:

```sh
nk --classification-help
```

Version information:

```sh
nk --version
```

Installed packages also provide the system manual page:

```sh
man nightkrawler
```

## Exit Status

Nightkrawler uses a non-zero exit status when an operation fails or when invalid command-line input is supplied.

Examples include:

- invalid option combinations
- missing required arguments
- failed Trash preflight
- failed quarantine operations
- refused protected-path actions
- failed restore or cleanup operations
- failed destructive actions

Successful operations return zero.

## Development Notes

The crate is built with Rust edition 2024.

For release builds, use the included lockfile:

```sh
cargo build --release --locked
```

The short `nk` command is not a separate binary. Packaged installations provide it as a symlink to `nightkrawler`.

## License

Nightkrawler is released under the BSD 3-Clause License.

See `LICENSE` for the full license text.

## Author

Markus Johnsson

## Related Projects

Nightkrawler belongs to the same family of command-line tools as Scry and Noct.
